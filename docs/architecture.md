# Architecture Technique de Kubunity

## 1. Vue d'Ensemble

**Kubunity** est conçu comme une plateforme de gestion déclarative opérée par cluster sous forme de **Helm Umbrella Chart modulaire**. 

L'approche repose sur un principe clé : **ne pas réinventer les briques d'infrastructure**, mais **orchestrer et abstraire les composants CNCF de référence** (Kyverno, External Secrets Operator, ArgoCD, Trivy Operator) sous une interface unifiée et des profils pré-configurés.

```
                    ┌────────────────────────────┐
                    │     Fleet Git Repository   │
                    │   (ApplicationSet / Helm)  │
                    └─────────────┬──────────────┘
                                  │
         ┌────────────────────────┼────────────────────────┐
         ▼                        ▼                        ▼
┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│   Cloud Cluster  │    │   Edge Cluster   │    │   Dev Cluster    │
│  (Profile Cloud) │    │  (Profile Edge)  │    │  (Profile Dev)   │
├──────────────────┤    ├──────────────────┤    ├──────────────────┤
│ • Kyverno        │    │ • Kyverno        │    │ • Kyverno        │
│   (Enforce)      │    │   (Audit)        │    │   (Audit Perm.)  │
│ • ESO (AWS/Vault)│    │ • ESO (Light)    │    │ • ESO (Mock/None)│
│ • Trivy Scanner  │    │ • Micro-Runtime  │    │ • Local FastLoop │
│ • Strict NetPol  │    │ • NetPol         │    │ • Dev Namespaces │
│ • OTel Collector │    │ • Light Metrics  │    │ • Local OTel/Logs│
│ • OpenObserve Hub│    │ • Edge Forwarder │    │ • Kusanagi Dev UI│
└──────────────────┘    └──────────────────┘    └──────────────────┘
```

---

## 2. Piliers d'Architecture

### 2.1 Modèle en Couches & Profils de Cluster

Chaque cluster Kubernetes géré par Kubunity est configuré via un fichier de profil (`values-*.yaml`) qui adapte le niveau d'exigence et les composants activés :

* **`cloud`** : Clusters de production ou de staging managés (EKS, GKE, AKS, OpenShift). Politiques en mode `Enforce`, intégrations avec les KMS/Secrets Manager cloud, monitoring avancé et scan de vulnérabilités activé.
* **`edge`** : Clusters distribués ou légers (K3s, IoT, Retail). Empreinte mémoire minimale, politiques en mode `Audit`, composants non essentiels désactivés.
* **`dev`** : Clusters locaux (Kind, K3d, Minikube). Prévu pour les développeurs, sans friction de validation d'image (`:latest` toléré) et déploiement instantané.

---

## 3. Flux d'Opération & GitOps

1. **Déclaration** : L'administrateur de plateforme déclare un nouveau cluster dans le dépôt GitOps avec son profil associé (`cloud`, `edge` ou `dev`).
2. **Synchronisation** : ArgoCD (ou Flux) détecte le nouveau cluster via l'`ApplicationSet` Kubunity et déploie le chart `kubunity` avec les valeurs du profil.
3. **Application de la Gouvernance** : Kyverno installe les politiques de sécurité (interdiction du root, quotas obligatoires, isolation réseau).
4. **Onboarding des Applications** : Les développeurs utilisent le blueprint `golden-paths/microservice` pour déployer leurs applications sans écrire de manifests complexes.
