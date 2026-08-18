# Guide de Démarrage Rapide (Quick Start)

Ce guide décrit comment déployer et tester le socle **Kubunity** sur un cluster local (Kind ou K3d) en moins de 5 minutes.

---

## Prérequis

* `helm` (v3.10+)
* `kubectl` (v1.28+)
* Un cluster Kubernetes opérationnel (ex: `k3d cluster create kubunity-lab` ou `kind create cluster --name kubunity-lab`)

---

## 1. Déploiement en Mode Local (Profil Dev)

Depuis la racine du dépôt `kubunity` :

```bash
# 1. Valider le template Helm avec le profil dev
helm template kubunity-dev ./charts/kubunity -f ./charts/kubunity/profiles/values-dev.yaml

# 2. Installer le socle Kubunity sur votre cluster
helm upgrade --install kubunity ./charts/kubunity \
  --namespace kubunity-system \
  --create-namespace \
  -f ./charts/kubunity/profiles/values-dev.yaml
```

---

## 2. Déployer une Application via le "Golden Path"

Pour déployer un microservice conforme aux standards de sécurité Kubunity :

```bash
helm upgrade --install my-service ./golden-paths/microservice \
  --namespace demo-app \
  --create-namespace \
  --set image.repository=nginx \
  --set image.tag=1.25.4-alpine
```

Vérifiez le statut du déploiement :
```bash
kubectl get pods,svc,netpol -n demo-app
```

---

## 3. Déploiement en Mode Production (Profil Cloud)

Pour un cluster de production (AWS EKS, GKE, AKS) :

```bash
helm upgrade --install kubunity ./charts/kubunity \
  --namespace kubunity-system \
  --create-namespace \
  -f ./charts/kubunity/profiles/values-cloud.yaml \
  --set cluster.name="prod-cluster-01"
```
