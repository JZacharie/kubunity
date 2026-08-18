#!/usr/bin/env bash
# ==============================================================================
# Kubunity CLI (kubunity-ctl)
# Outil de contrôle, de validation et de déploiement pour la plateforme Kubunity
# ==============================================================================

set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

# Couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

function print_banner() {
    echo -e "${CYAN}"
    echo "  _  ___     _                 _ _ "
    echo " | |/ / |   | |               (_) |"
    echo " | ' /| |___| |__  _   _ _ __  _| |_ _   _ "
    echo " |  < | '_  | '_ \| | | | '_ \| | __| | | |"
    echo " | . \| |_) | |_) | |_| | | | | | |_| |_| |"
    echo " |_|\_\_.__/|_.__/ \__,_|_| |_|_|\__|\__, |"
    echo "                                      __/ |"
    echo "                                     |___/ "
    echo -e "  Unified & Declarative Kubernetes Platform${NC}"
    echo ""
}

function usage() {
    print_banner
    echo -e "${YELLOW}Usage:${NC} $0 <command> [options]"
    echo ""
    echo -e "${BLUE}Commandes disponibles :${NC}"
    echo "  build                  Télécharger et compiler les dépendances Helm CNCF"
    echo "  lint                   Exécuter helm lint sur tous les charts et profils"
    echo "  template [profile]     Valider le rendu des templates (dev|cloud|edge|all)"
    echo "  audit                  Audit de sécurité et de conformité des manifests rendus"
    echo "  install <profile>      Installer ou mettre à jour Kubunity sur le cluster actif"
    echo "  status                 Afficher l'état du déploiement Kubunity sur le cluster"
    echo "  help                   Afficher ce message d'aide"
    echo ""
    echo -e "${BLUE}Exemples :${NC}"
    echo "  $0 build"
    echo "  $0 lint"
    echo "  $0 template cloud"
    echo "  $0 install dev"
    echo ""
}

function check_dependencies() {
    if ! command -v helm &> /dev/null; then
        echo -e "${RED}Erreur : 'helm' n'est pas installé.${NC}"
        exit 1
    fi
}

function cmd_build() {
    echo -e "${CYAN}==> Mise à jour et compilation des dépendances Helm...${NC}"
    helm repo add kyverno https://kyverno.github.io/kyverno/ &> /dev/null || true
    helm repo add external-secrets https://charts.external-secrets.io &> /dev/null || true
    helm repo add aqua https://aquasecurity.github.io/helm-charts/ &> /dev/null || true
    helm repo add argo-cd https://argoproj.github.io/argo-helm &> /dev/null || true
    helm repo update &> /dev/null || true
    helm dependency build "${ROOT_DIR}/charts/kubunity"
    echo -e "${GREEN}✓ Dépendances compilées avec succès.${NC}"
}

function cmd_lint() {
    echo -e "${CYAN}==> Exécution de Helm Lint sur Kubunity...${NC}"
    helm lint "${ROOT_DIR}/charts/kubunity"
    helm lint "${ROOT_DIR}/charts/kubunity" -f "${ROOT_DIR}/charts/kubunity/profiles/values-dev.yaml"
    helm lint "${ROOT_DIR}/charts/kubunity" -f "${ROOT_DIR}/charts/kubunity/profiles/values-cloud.yaml"
    helm lint "${ROOT_DIR}/charts/kubunity" -f "${ROOT_DIR}/charts/kubunity/profiles/values-edge.yaml"
    helm lint "${ROOT_DIR}/golden-paths/microservice"
    echo -e "${GREEN}✓ Tous les charts et profils sont valides.${NC}"
}

function cmd_template() {
    local profile="${1:-all}"
    echo -e "${CYAN}==> Rendu des templates pour le profil : ${profile}...${NC}"

    case "$profile" in
        dev)
            helm template test-kubunity "${ROOT_DIR}/charts/kubunity" -f "${ROOT_DIR}/charts/kubunity/profiles/values-dev.yaml" > /dev/null
            ;;
        cloud)
            helm template test-kubunity "${ROOT_DIR}/charts/kubunity" -f "${ROOT_DIR}/charts/kubunity/profiles/values-cloud.yaml" > /dev/null
            ;;
        edge)
            helm template test-kubunity "${ROOT_DIR}/charts/kubunity" -f "${ROOT_DIR}/charts/kubunity/profiles/values-edge.yaml" > /dev/null
            ;;
        all)
            helm template test-dev "${ROOT_DIR}/charts/kubunity" -f "${ROOT_DIR}/charts/kubunity/profiles/values-dev.yaml" > /dev/null
            helm template test-cloud "${ROOT_DIR}/charts/kubunity" -f "${ROOT_DIR}/charts/kubunity/profiles/values-cloud.yaml" > /dev/null
            helm template test-edge "${ROOT_DIR}/charts/kubunity" -f "${ROOT_DIR}/charts/kubunity/profiles/values-edge.yaml" > /dev/null
            helm template test-app "${ROOT_DIR}/golden-paths/microservice" > /dev/null
            ;;
        *)
            echo -e "${RED}Profil inconnu: ${profile}. Utilisez dev, cloud, edge, ou all.${NC}"
            exit 1
            ;;
    esac
    echo -e "${GREEN}✓ Rendu des templates validé sans erreur.${NC}"
}

function cmd_audit() {
    echo -e "${CYAN}==> Audit de conformité de sécurité (Pod Security Standards Restricted)...${NC}"
    local tmp_output
    tmp_output=$(mktemp)
    helm template test-cloud "${ROOT_DIR}/charts/kubunity" -f "${ROOT_DIR}/charts/kubunity/profiles/values-cloud.yaml" > "${tmp_output}"
    helm template test-app "${ROOT_DIR}/golden-paths/microservice" >> "${tmp_output}"

    echo -e "${BLUE}1. Vérification des conteneurs non-root (runAsNonRoot)...${NC}"
    if grep -q "runAsNonRoot: true" "${tmp_output}"; then
        echo -e "${GREEN}  ✓ runAsNonRoot configuré.${NC}"
    else
        echo -e "${YELLOW}  ⚠ Attention : runAsNonRoot non détecté.${NC}"
    fi

    echo -e "${BLUE}2. Vérification des politiques Kyverno PSS...${NC}"
    local policies_count
    policies_count=$(grep -c "kind: ClusterPolicy" "${tmp_output}" || true)
    echo -e "${GREEN}  ✓ ${policies_count} politiques Kyverno actives détectées.${NC}"

    echo -e "${BLUE}3. Vérification du collecteur OpenTelemetry & OpenObserve...${NC}"
    if grep -q "otelcol" "${tmp_output}"; then
        echo -e "${GREEN}  ✓ Pipeline OpenTelemetry configuré.${NC}"
    fi

    rm -f "${tmp_output}"
    echo -e "${GREEN}✓ Audit terminé avec succès.${NC}"
}

function cmd_install() {
    local profile="${1:-dev}"
    local profile_file="${ROOT_DIR}/charts/kubunity/profiles/values-${profile}.yaml"

    if [ ! -f "${profile_file}" ]; then
        echo -e "${RED}Fichier de profil introuvable : ${profile_file}${NC}"
        exit 1
    fi

    echo -e "${CYAN}==> Déploiement de Kubunity sur le cluster actif avec le profil [${profile}]...${NC}"
    helm upgrade --install kubunity "${ROOT_DIR}/charts/kubunity" \
        --namespace kubunity-system \
        --create-namespace \
        -f "${profile_file}"

    echo -e "${GREEN}✓ Kubunity déployé avec succès dans le namespace kubunity-system.${NC}"
}

function cmd_status() {
    if ! command -v kubectl &> /dev/null; then
        echo -e "${RED}Erreur : 'kubectl' n'est pas installé.${NC}"
        exit 1
    fi

    echo -e "${CYAN}==> Statut de Kubunity sur le cluster actif :${NC}"
    kubectl get all,clusterpolicies -n kubunity-system 2>/dev/null || echo -e "${YELLOW}Aucune ressource trouvée dans kubunity-system.${NC}"
}

# Point d'entrée principal
check_dependencies

case "${1:-help}" in
    build)
        cmd_build
        ;;
    lint)
        cmd_lint
        ;;
    template)
        cmd_template "$2"
        ;;
    audit)
        cmd_audit
        ;;
    install)
        cmd_install "$2"
        ;;
    status)
        cmd_status
        ;;
    help|--help|-h)
        usage
        ;;
    *)
        echo -e "${RED}Commande inconnue: $1${NC}"
        usage
        exit 1
        ;;
esac
