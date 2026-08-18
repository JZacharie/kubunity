# 🤝 Guide de Contribution & Normes de Développement

Merci pour votre intérêt à contribuer à **Kubunity** ! Ce document définit les standards techniques, les processus de test et les conventions de code pour maintenir une plateforme robuste, sécurisée et évolutive.

---

## 🛠️ 1. Environnement de Développement & Outils

Pour développer sur Kubunity, vous devez disposer des outils suivants installés localement :

| Outil | Version Minimale | Rôle |
| :--- | :--- | :--- |
| **`helm`** | `v3.14.0+` | Packaging et moteur de template des charts. |
| **`kubectl`** | `v1.28.0+` | Interaction avec le cluster Kubernetes. |
| **`k3d`** ou **`kind`** | Récent | Création de clusters Kubernetes locaux éphémères pour les tests. |
| **`git`** | `v2.40+` | Gestion de versions. |

---

## 📁 2. Structure et Conventions du Dépôt

* **`charts/kubunity/`** : Chart Helm Umbrella principal (dépendances CNCF, politiques, observabilité).
* **`charts/kubunity/profiles/`** : Fichiers de surcharges par environnement (`values-dev.yaml`, `values-cloud.yaml`, `values-edge.yaml`).
* **`golden-paths/`** : Modèles applicatifs prêts à l'emploi pour les développeurs.
* **`docs/`** : Documentation technique et guides d'architecture.
* **`scripts/`** : Utilitaires d'administration et de test (`kubunity-ctl.sh`).

---

## 🔒 3. Règles d'Or pour l'Ajout de Fonctionnalités

### A. Règle du "Secure by Default"
* Tout nouveau template ou Golden Path doit impérativement respecter les règles **Pod Security Standards (Restricted)** :
  * `runAsNonRoot: true`
  * `allowPrivilegeEscalation: false`
  * `readOnlyRootFilesystem: true`
  * `capabilities: drop: ["ALL"]`
* Exiger systématiquement la définition des requêtes (`requests`) et limites (`limits`) de ressources CPU et mémoire.

### B. Règle de l'Abstraction Déclarative
* Ne jamais forcer un utilisateur à écrire du YAML bas niveau complexe s'il peut être abstrait par un booléen ou un bloc simple dans `values.yaml` (ex: `security.kyverno.enforceBaseline: true`).
* Les profils (`dev`, `cloud`, `edge`) doivent rester cohérents et couvrir les 3 cas d'usage majeurs.

### C. Règles pour les Politiques Kyverno (`charts/kubunity/templates/policies/`)
* Chaque politique Kyverno doit être encapsulée dans une condition Helm (ex: `{{- if and .Values.security.kyverno.enabled .Values.security.kyverno.policies.myPolicy.enabled }}`).
* Exclure obligatoirement les namespaces système K8s (`kube-system`, `kube-public`, `kube-node-lease`).

---

## 🧪 4. Processus de Validation Locale

Avant de soumettre une modification ou une Pull Request, exécutez la suite de tests avec l'outil de contrôle `kubunity-ctl` :

```bash
# 1. Vérifier la syntaxe et le lint de tous les charts et profils
./scripts/kubunity-ctl.sh lint

# 2. Vérifier le rendu des templates pour tous les profils
./scripts/kubunity-ctl.sh template

# 3. Exécuter un audit de conformité de sécurité local
./scripts/kubunity-ctl.sh audit
```

---

## 🚀 5. Cycle de Vie des Releases & Conventions Git

* **Messages de Commit (Conventional Commits)** :
  * `feat: ...` : Nouvelle fonctionnalité ou template.
  * `fix: ...` : Correction de bug ou de manifest.
  * `docs: ...` : Modification de documentation.
  * `ci: ...` : Modification des workflows GitHub Actions.
  * `chore: ...` : Maintenance ou mise à jour de dépendances.

* **Workflow de Release** :
  * Les tags Git respectent le versionnage sémantique (`vX.Y.Z`).
  * Chaque push sur `main` déclenche le workflow de validation CI.
  * Chaque création de tag déclenche la publication automatique des artefacts OCI sur GitHub Container Registry (`ghcr.io`).
