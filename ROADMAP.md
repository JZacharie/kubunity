# 🗺️ Feuille de Route Stratégique & Technique (Roadmap) : Kubunity

Ce document détaille les phases d'ingénierie, jalons et livrables pour faire évoluer **Kubunity** d'un socle Helm unifié vers une plateforme d'entreprise complète pour la gestion déclarative de flottes Kubernetes.

---

## 📅 Synthèse des Phases de Développement

| Phase | Horizon | Cible SemVer | Focus Principal |
| :--- | :--- | :--- | :--- |
| **Phase 1** | Sprint 1 | `v0.2.0` | **Packaging OCI, CLI d'Administration & CI End-to-End** |
| **Phase 2** | Sprint 2 | `v0.3.0` | **Golden Paths Étendus (CloudNativePG & AI Gateway)** |
| **Phase 3** | Sprint 3 | `v0.4.0` | **FinOps (OpenCost), Nettoyage Éphémère & Télémétrie Avancée** |
| **Phase 4** | Sprint 4 | `v1.0.0` | **Fédération Multi-Clusters (Cilium Mesh) & Vue Flotte Kusanagi** |

---

## 📦 Phase 1 : Packaging OCI, Automation CLI & Tests E2E (`v0.2.0`)

### 1.1 Objectifs
* Permettre la distribution de Kubunity sous forme d'artefacts OCI (`oci://ghcr.io/jzacharie/charts/kubunity`).
* Fournir une CLI locale d'administration et de diagnostic rapide (`scripts/kubunity-ctl.sh`).
* Valider les déploiements réels sur cluster éphémère (Kind / K3d) dans la CI.

### 1.2 Tâches Techniques
- [x] Créer le script d'administration `scripts/kubunity-ctl.sh` (commandes: `install`, `lint`, `template`, `audit`, `status`).
- [ ] Créer le workflow GitHub Actions `.github/workflows/release-oci.yaml` pour packager et publier sur GHCR.
- [ ] Mettre en place un job de test E2E sur cluster Kind éphémère dans la CI.
- [ ] Rédiger les guides de contribution dans `CONTRIBUTING.md`.

---

## 🗄️ Phase 2 : Catalogue Étendu de Golden Paths Applicatifs (`v0.3.0`)

### 2.1 Objectifs
* Dépasser le simple déploiement de microservices stateless en fournissant des blueprints prêts pour la production pour la persistance et les charges IA (inspiré de `jo3`).

### 2.2 Tâches Techniques
- [ ] **Golden Path Database (`golden-paths/database`)** :
  * Intégration de l'opérateur **CloudNativePG**.
  * Déploiement d'un cluster PostgreSQL HA avec 3 instances répliquées.
  * Sauvegardes automatisées S3/MinIO avec chiffrement et rétention.
- [ ] **Golden Path AI Gateway (`golden-paths/ai-gateway`)** :
  * Proxy LLM basé sur Bifrost / LiteLLM avec routage intelligent et fallbacks.
  * Rate-limiting par API Key et suivi de consommation de tokens exporté dans OpenObserve.
- [ ] **Golden Path Frontend (`golden-paths/frontend`)** :
  * Blueprint pour applications React/Next.js/SPA avec mise en cache NGINX/Traefik et TLS automatisé.

---

## 💰 Phase 3 : FinOps & Cycle de Vie Éphémère (`v0.4.0`)

### 3.1 Objectifs
* Donner une visibilité financière complète sur les coûts d'infrastructure par tenant/namespace et automatiser le nettoyage des ressources non utilisées.

### 3.2 Tâches Techniques
- [ ] **Intégration OpenCost (`finops.opencost.enabled: true`)** :
  * Déploiement de l'exportateur OpenCost corrélé aux métriques Prometheus.
  * Remontée des données de coûts directement dans le dashboard **Kusanagi**.
- [ ] **Gestion des Espaces Éphémères & TTL Cleaner** :
  * Politique Kyverno de suppression automatique des namespaces de preview/développement après `X` jours d'inactivité.
- [ ] **Rapports d'Optimisation des Ressources** :
  * Détection automatique des conteneurs sur-dimensionnés ou sous-utilisés via les métriques OTel.

---

## 🌐 Phase 4 : Fédération Multi-Clusters & Réseau Global (`v1.0.0`)

### 4.1 Objectifs
* Transformer une flotte de clusters indépendants en un continuum sécurisé et interconnecté.

### 4.2 Tâches Techniques
- [ ] **Cilium Cluster Mesh** :
  * Interconnexion chiffrée (WireGuard) inter-clusters pour communication service-to-service transparente entre Cloud et Edge.
- [ ] **Agrégation Multi-Clusters dans Kusanagi** :
  * Vue globale unifiée dans Kusanagi permettant de visualiser et d'interagir avec tous les clusters de la flotte.
- [ ] **Site de Documentation Interactif** :
  * Déploiement d'un site de documentation MkDocs / Docusaurus sur GitHub Pages.
