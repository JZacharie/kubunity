# 🤖 Directives pour Agents IA & Développeurs : Kubunity

Ce fichier définit les règles opérationnelles, les contraintes d'architecture et les procédures de validation pour tout agent IA (ou contributeur) intervenant sur le dépôt **Kubunity**.

---

## 🏛️ 1. Principes d'Architecture Inviolables

1. **Abstraction Déclarative** :
   * Ne jamais forcer l'utilisateur à manipuler des dizaines de fichiers YAML disparates.
   * Tout nouveau composant CNCF doit être activable via un bloc booléen clair dans [charts/kubunity/values.yaml](charts/kubunity/values.yaml).
2. **Secure by Default (NSA / CISA / PSS Restricted)** :
   * Tout workload ou template applicatif doit définir :
     * `runAsNonRoot: true`
     * `allowPrivilegeEscalation: false`
     * `readOnlyRootFilesystem: true`
     * `capabilities: drop: ["ALL"]`
   * Quotas CPU/RAM (`requests` et `limits`) obligatoires sur tout conteneur.
3. **Observabilité Intégrée** :
   * Tout service applicatif ou composant déployé doit être compatible avec l'export de métriques/traces vers l'OpenTelemetry Collector local (`kubunity-otel-collector:4318`) et OpenObserve.
4. **Cohérence des 3 Profils** :
   * Toute modification de `values.yaml` doit être répercutée ou vérifiée sur :
     * `charts/kubunity/profiles/values-dev.yaml` (Local Kind/K3d)
     * `charts/kubunity/profiles/values-cloud.yaml` (Cloud EKS/GKE/AKS durci)
     * `charts/kubunity/profiles/values-edge.yaml` (Edge K3s minimal)

---

## 🧪 2. Procédure de Validation Obligatoire

Avant de commiter ou de proposer une modification, vous **DEVEZ** exécuter les vérifications suivantes via `scripts/kubunity-ctl.sh` :

```bash
# 1. Compilation des dépendances Helm
./scripts/kubunity-ctl.sh build

# 2. Linting syntaxique de tous les charts et profils
./scripts/kubunity-ctl.sh lint

# 3. Rendu des templates pour tous les profils
./scripts/kubunity-ctl.sh template all

# 4. Audit de sécurité automatisé
./scripts/kubunity-ctl.sh audit
```

---

## 📝 3. Format des Messages de Commit

Utilisez impérativement la convention **Conventional Commits** :
* `feat:` Nouvelle fonctionnalité (ex: nouveau template Golden Path, nouveau module d'observabilité).
* `fix:` Correction de manifest Helm, politique Kyverno ou bug CI.
* `docs:` Mise à jour de la documentation, du README ou de la ROADMAP.
* `ci:` Modification des workflows GitHub Actions.
* `chore:` Mise à jour des dépendances Helm ou du `.gitignore`.
