# 🌐 Kubunity : Unified & Declarative Kubernetes Platform

[![Kubernetes](https://img.shields.io/badge/Kubernetes-v1.28+-326ce5?style=for-the-badge&logo=kubernetes&logoColor=white)](https://kubernetes.io)
[![Helm](https://img.shields.io/badge/Helm-v3.14+-0f1689?style=for-the-badge&logo=helm&logoColor=white)](https://helm.sh)
[![GitOps](https://img.shields.io/badge/GitOps-ArgoCD-orange?style=for-the-badge&logo=argo&logoColor=white)](https://argoproj.github.io)
[![Policy Engine](https://img.shields.io/badge/Policy-Kyverno-00B4D8?style=for-the-badge&logo=kyverno&logoColor=white)](https://kyverno.io)
[![Observability](https://img.shields.io/badge/Telemetry-OTel_%2B_OpenObserve-0077B6?style=for-the-badge&logo=opentelemetry&logoColor=white)](https://openobserve.ai)
[![Security](https://img.shields.io/badge/Security-Trivy_%2B_ESO-blueviolet?style=for-the-badge&logo=aquasecurity&logoColor=white)](https://external-secrets.io)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)

> **[🇬🇧 English Version](#-english-version)** | **[🇫🇷 Version Française](#-version-française)**

---

# 🇬🇧 English Version

## 1. Overview & Vision

**Kubunity** is an opinionated, cloud-native **Helm Umbrella & Platform solution** designed to unify multi-cluster Kubernetes fleets. It abstracts complex CNCF components into a simplified declarative interface, delivering:
* 🚀 **Multi-Cluster Profiles**: Turnkey configurations for Cloud (`EKS`/`GKE`/`AKS`), Edge (`K3s`/`IoT`), and Local Dev (`Kind`/`K3d`).
* 🛡️ **Hardened Policy-as-Code**: Pre-packaged Kyverno rules enforcing **NSA/CISA & Pod Security Standards (Restricted)**.
* 📊 **Unified Observability**: End-to-end telemetry pipeline via **OpenTelemetry Collector** and **OpenObserve** (Logs, Metrics, Traces).
* 🔮 **Real-time AI Control Plane**: Native integration with **Kusanagi** (Rust/Axum real-time dashboard & LLM cluster assistant).
* 🔐 **Centralized Secrets & Zero Trust**: External Secrets Operator (ESO) integration with Vault/AWS/GCP/Azure + Cilium eBPF L7 network policies.
* 📦 **Developer Golden Paths**: Production-ready microservice blueprints deployable in under 15 lines of YAML.

---

## 2. Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          KUBUNITY HELM PLATFORM                             │
│                     (Single unified values.yaml interface)                  │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
      ┌─────────────────┬──────────────┴──────────────┬──────────────────┐
      ▼                 ▼                             ▼                  ▼
┌─────────────┐ ┌───────────────┐             ┌───────────────┐ ┌───────────────┐
│   GitOps    │ │  Governance   │             │ Observability │ │  Golden Paths │
├─────────────┤ ├───────────────┤             ├───────────────┤ ├───────────────┤
│ • ArgoCD    │ │ • Kyverno PSS │             │ • OTel Relay  │ │ • Microservice│
│ • AppSets   │ │ • ESO / Vault │             │ • OpenObserve │ │ • Auto Tenant │
│ • Multi-Sync│ │ • Trivy Audit │             │ • Kusanagi AI │ │   Namespace   │
│ • Sync Waves│ │ • Cilium eBPF │             │ • Hubble L7   │ │   & Quotas    │
└─────────────┘ └───────────────┘             └───────────────┘ └───────────────┘
```

---

## 3. Key Modules & CNCF Stack

| Module | Component | Capabilities |
| :--- | :--- | :--- |
| **GitOps** | ArgoCD + ApplicationSets | Fleet matrix synchronization, auto self-healing, multi-tenant `AppProjects`. |
| **Policy Engine** | Kyverno (v3.8+) | Rootless enforcement, CPU/RAM quotas, image tag immutability, zero-trust NetPol generation. |
| **Telemetry** | OpenTelemetry Collector | OTLP (gRPC 4317 / HTTP 4318), Prometheus scraping, K8s attributes enrichment. |
| **Analytics Engine**| OpenObserve | Parquet-backed high-speed indexing for distributed Logs, Metrics, and Traces. |
| **Dashboard & AI** | Kusanagi | Real-time PWA monitoring in Rust/Axum + LLM contextual cluster troubleshooting. |
| **Networking** | Cilium (eBPF) | Layer 7 DNS filtering & cluster-wide zero-trust network policies. |
| **Secrets Engine** | External Secrets Operator | Dynamic injection from HashiCorp Vault, AWS Secrets Manager, GCP Secret Manager, Azure Key Vault. |

---

## 4. Quick Start (5 Minutes)

### Prerequisites
* `helm` (v3.10+)
* `kubectl` (v1.28+)
* A running Kubernetes cluster (`k3d`, `kind`, `minikube`, or cloud)

### Installation

```bash
# 1. Clone Kubunity
git clone https://github.com/JZacharie/kubunity.git
cd kubunity

# 2. Build Helm dependencies
helm repo add kyverno https://kyverno.github.io/kyverno/
helm repo add external-secrets https://charts.external-secrets.io
helm repo add aqua https://aquasecurity.github.io/helm-charts/
helm repo add argo-cd https://argoproj.github.io/argo-helm
helm dependency build charts/kubunity

# 3. Deploy using your cluster profile (dev, cloud, or edge)
helm upgrade --install kubunity ./charts/kubunity \
  --namespace kubunity-system \
  --create-namespace \
  -f ./charts/kubunity/profiles/values-dev.yaml
```

### Deploy an Application via Golden Path
```bash
helm upgrade --install my-app ./golden-paths/microservice \
  --namespace demo-app \
  --create-namespace \
  --set image.repository=nginx \
  --set image.tag=1.25.4-alpine
```

---

## 5. Cluster Profiles

* ☁️ **`values-cloud.yaml`** : Production-hardened mode with `Enforce` Kyverno policies, OpenTelemetry Collector, OpenObserve hub, Cilium eBPF L7, and Cloud KMS secrets.
* 🌐 **`values-edge.yaml`** : Ultra-low footprint tailored for K3s / IoT / Edge environments.
* 💻 **`values-dev.yaml`** : Permissive development setup for fast local loops with Kind/K3d and instant Kusanagi dashboard.

---
---

# 🇫🇷 Version Française

## 1. Vision & Présentation

**Kubunity** est une solution de plateforme déclarative opérée par cluster sous la forme d'un **Umbrella Helm Chart modulaire**. Elle unifie la gestion de flottes Kubernetes hétérogènes en encapsulant et simplifiant le meilleur de l'écosystème CNCF :

* 🚀 **Profils Multi-Clusters Clé en Main** : Configurations optimisées pour le Cloud (`EKS`/`GKE`/`AKS`), l'Edge (`K3s`/`IoT`) et le Dev local (`Kind`/`K3d`).
* 🛡️ **Gouvernance & Sécurité Renforcée** : Pack de règles Kyverno pré-intégrées respectant les standards **NSA/CISA & Pod Security Standards (Restricted)**.
* 📊 **Observabilité Unifiée** : Pipeline télémétrique complet via **OpenTelemetry Collector** et **OpenObserve** (Logs, Métriques, Traces).
* 🔮 **Dashboard & Assistant IA Temps Réel** : Intégration native avec **Kusanagi** (Dashboard PWA en Rust/Axum + Assistant LLM d'analyse de cluster).
* 🔐 **Secrets Centralisés & Zero Trust** : Orchestration via External Secrets Operator (ESO) relié à Vault/KMS Cloud + politiques réseau L7 Cilium eBPF.
* 📦 **Golden Paths Développeurs** : Blueprints de microservices standardisés prêts pour la production en moins de 15 lignes de YAML.

---

## 2. Schéma d'Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          PLATEFORME HELM KUBUNITY                           │
│                   (Point d'entrée unique values.yaml)                       │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
      ┌─────────────────┬──────────────┴──────────────┬──────────────────┐
      ▼                 ▼                             ▼                  ▼
┌─────────────┐ ┌───────────────┐             ┌───────────────┐ ┌───────────────┐
│   GitOps    │ │  Gouvernance  │             │ Observabilité │ │  Golden Paths │
├─────────────┤ ├───────────────┤             ├───────────────┤ ├───────────────┤
│ • ArgoCD    │ │ • Kyverno PSS │             │ • OTel Relay  │ │ • Microservice│
│ • AppSets   │ │ • ESO / Vault │             │ • OpenObserve │ │ • Onboarding  │
│ • Multi-Sync│ │ • Audit Trivy │             │ • Kusanagi IA │ │   Tenants &   │
│ • Sync Waves│ │ • Cilium eBPF │             │ • Hubble L7   │ │   Quotas      │
└─────────────┘ └───────────────┘             └───────────────┘ └───────────────┘
```

---

## 3. Les Briques CNCF Intégrées

| Module | Composant | Fonctionnalités Clés |
| :--- | :--- | :--- |
| **GitOps** | ArgoCD + ApplicationSets | Synchronisation matricielle de flotte, auto-healing, génération d'`AppProjects` multi-tenants. |
| **Moteur de Politiques** | Kyverno (v3.8+) | Interdiction du root, quotas CPU/RAM, immutabilité des tags d'images, génération automatique de NetworkPolicies. |
| **Télémétrie** | OpenTelemetry Collector | Réception OTLP (gRPC 4317 / HTTP 4318), scraping Prometheus, enrichissement automatique des métadonnées K8s. |
| **Moteur Analytique** | OpenObserve | Stockage haute performance Parquet et indexation rapide des Logs, Métriques et Traces. |
| **Dashboard & IA** | Kusanagi | Surveillance PWA temps réel ultra-rapide en Rust/Axum + assistant IA d'investigation de cluster. |
| **Réseau eBPF** | Cilium + Hubble | Filtrage DNS Layer 7 et politiques réseau zero-trust globales (`CiliumClusterwideNetworkPolicy`). |
| **Gestion des Secrets** | External Secrets Operator | Injection dynamique depuis HashiCorp Vault, AWS Secrets Manager, GCP Secret Manager, Azure Key Vault. |

---

## 4. Démarrage Rapide (5 Minutes)

### Prérequis
* `helm` (v3.10+)
* `kubectl` (v1.28+)
* Un cluster Kubernetes actif (`k3d`, `kind`, `minikube` ou cluster distant)

### Déploiement du Socle

```bash
# 1. Cloner le dépôt
git clone https://github.com/JZacharie/kubunity.git
cd kubunity

# 2. Télécharger les dépendances Helm
helm repo add kyverno https://kyverno.github.io/kyverno/
helm repo add external-secrets https://charts.external-secrets.io
helm repo add aqua https://aquasecurity.github.io/helm-charts/
helm repo add argo-cd https://argoproj.github.io/argo-helm
helm dependency build charts/kubunity

# 3. Déployer selon le profil souhaité (dev, cloud, edge)
helm upgrade --install kubunity ./charts/kubunity \
  --namespace kubunity-system \
  --create-namespace \
  -f ./charts/kubunity/profiles/values-dev.yaml
```

### Déployer une Application via le Golden Path
```bash
helm upgrade --install my-app ./golden-paths/microservice \
  --namespace demo-app \
  --create-namespace \
  --set image.repository=nginx \
  --set image.tag=1.25.4-alpine
```

---

## 5. Profils de Déploiement

* ☁️ **`values-cloud.yaml`** : Profil durci pour la production (EKS/GKE/AKS/OpenShift) avec politiques Kyverno en mode `Enforce`, collecteur OTel, OpenObserve, Cilium eBPF L7 et KMS Cloud.
* 🌐 **`values-edge.yaml`** : Profil ultra-léger optimisé pour les environnements K3s, IoT et Edge.
* 💻 **`values-dev.yaml`** : Profil permissif pour tests rapides en local (Kind / K3d) avec dashboard Kusanagi instantané.

---

## 🤝 Contribution & Licence

Les contributions sont les bienvenues ! Consultez la documentation technique dans [`docs/`](docs/) pour en savoir plus.

Distribué sous licence **MIT**.
