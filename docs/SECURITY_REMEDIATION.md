# 🛡️ Guide de Remédiation de Sécurité & Base Images Durcies (Cluster jo3)

Ce guide décrit la stratégie de remédiation automatisée et les modèles de Dockerfiles durcis permettant de corriger plus de **600 occurrences de failles critiques** sur le cluster Kubernetes en ciblant les images de base.

---

## 📊 1. Analyse des Failles Critiques du Cluster (jo3)

* **Détections totales (Occurrences brutes)** : **1 168** alertes critiques.
* **CVEs Critiques Uniques (Distinctes)** : **273** CVEs.
* **Taux de Redondance** : **76.6%** des alertes critiques proviennent des mêmes bibliothèques partagées (`crypto/tls`, `openssl`, `node-tar`, `gnutls`).

### Top 5 des CVEs Critiques Résolues par la Mise à Jour des Images de Base :

1. **`CVE-2025-68121`** (120 conteneurs) : Go `crypto/tls` ➔ Résolu avec `golang:1.24-alpine3.21` / rebuild.
2. **`CVE-2026-31789`** (108 conteneurs) : OpenSSL Heap buffer overflow ➔ Résolu avec `alpine:3.21` (`libssl3` patché).
3. **`CVE-2026-33186`** (52 conteneurs) : `grpc-go` Authz bypass ➔ Résolu avec `golang:1.24`.
4. **`CVE-2026-59873`** (40 conteneurs) : `node-tar` DoS ➔ Résolu avec `node:22-alpine3.21`.
5. **`CVE-2026-33845` & `CVE-2026-42010`** (68 conteneurs) : GnuTLS DTLS / NUL bypass ➔ Résolu avec `debian:bookworm-slim` / `alpine:3.21`.

---

## 🏗️ 2. Modèles de Dockerfiles Golden Paths Durcis (Zero-CVE)

Les modèles prêts pour la production sont situés dans [`golden-paths/docker/`](../golden-paths/docker/) :

### A. Go (Distroless Non-Root) : [`Dockerfile.go-hardened`](../golden-paths/docker/Dockerfile.go-hardened)
* **Base de build** : `golang:1.24-alpine3.21`
* **Base de runtime** : `gcr.io/distroless/static-debian12:nonroot`
* **Utilisateur** : Non-root (`UID 65532`)
* **Binaire** : Statique CGO_ENABLED=0 sans dépendance dynamique.

### B. Rust (Musl Static + Distroless) : [`Dockerfile.rust-hardened`](../golden-paths/docker/Dockerfile.rust-hardened)
* **Base de build** : `rust:1.85-alpine3.21` + `cargo-chef` pour mise en cache optimale.
* **Base de runtime** : `gcr.io/distroless/static-debian12:nonroot`
* **Utilisateur** : Non-root (`UID 65532`)

### C. Node.js (Alpine 3.21) : [`Dockerfile.node-hardened`](../golden-paths/docker/Dockerfile.node-hardened)
* **Base de runtime** : `node:22-alpine3.21`
* **Gestionnaire d'init** : Tini (`/sbin/tini`) pour la gestion des signaux PID 1.
* **Utilisateur** : Non-root (`node:node` UID 1000)

### D. Python (Debian Bookworm-Slim) : [`Dockerfile.python-hardened`](../golden-paths/docker/Dockerfile.python-hardened)
* **Base de runtime** : `python:3.12-slim-bookworm`
* **Environnement** : Virtualenv `/opt/venv` pré-compilé.
* **Utilisateur** : Non-root (`appuser` UID 10001)

---

## ⚡ 3. Instructions de Déploiement & Rebuild

Pour mettre à jour et nettoyer les vulnérabilités de conteneurs sur vos microservices :

```bash
# 1. Tester la conformité du chart Helm avec le nouveau tag
./scripts/kubunity-ctl.sh lint

# 2. Reconstruire et scanner une image avec Trivy
docker build -t my-app:v1.0.0 -f golden-paths/docker/Dockerfile.go-hardened .
trivy image --severity CRITICAL,HIGH my-app:v1.0.0

# 3. Déployer sur le cluster via Kubunity ou ArgoCD
```
