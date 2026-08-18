# Gouvernance & Sécurité dans Kubunity

## 1. Principes Fondamentaux

Kubunity applique le principe de **Sécurité par Défaut** (*Secure-by-Default*) et de **Défense en Profondeur**.

---

## 2. Politiques Kyverno Pré-packagées

| Politique | Mode Recommandé | Description |
| :--- | :--- | :--- |
| `kubunity-disallow-root` | `Enforce` (Prod) / `Audit` (Dev) | Interdit l'exécution de conteneurs en tant que root (`runAsNonRoot: true`). |
| `kubunity-require-resource-requests-limits` | `Enforce` (Prod) / `Audit` (Dev) | Exige la présence de quotas CPU et RAM pour garantir la stabilité du cluster. |
| `kubunity-disallow-latest-tag` | `Enforce` (Prod) | Bloque l'usage de tags mutables `:latest` en production. |
| `kubunity-generate-default-network-policy` | Actif | Génère automatiquement une `NetworkPolicy` d'isolation pour tout nouveau namespace. |
| `kubunity-disallow-privilege-escalation` | `Enforce` (Prod) | Bloque l'élévation de privilèges (`allowPrivilegeEscalation: false`). |
| `kubunity-disallow-host-namespaces` | `Enforce` (Prod) | Bloque l'accès aux namespaces hôte (`hostNetwork`, `hostPID`, `hostIPC`). |
| `kubunity-require-readonly-rootfs` | `Enforce` (Prod) | Exige le système de fichiers racine en lecture seule (`readOnlyRootFilesystem: true`). |
| `kubunity-restrict-cluster-admin-bindings` | `Enforce` (Prod) | Interdit l'attribution de `cluster-admin` aux ServiceAccounts applicatifs. |

---

## 3. Réseau eBPF & Cilium L7 Inspection

Kubunity intègre des politiques `CiliumClusterwideNetworkPolicy` permettant l'inspection au niveau applicatif (Layer 7 DNS, HTTP, gRPC) et l'isolation zero-trust eBPF.

---

## 4. Dashboard & Observabilité IA Kusanagi

Kubunity propose l'intégration native de **Kusanagi** (`observability.kusanagi.enabled: true`), offrant un tableau de bord temps réel PWA et un assistant IA connecté à la flotte de clusters.

---

## 5. Gestion des Secrets avec External Secrets Operator (ESO)

Kubunity utilise `ExternalSecrets` pour éviter de stocker des secrets en clair dans Git :

```yaml
apiVersion: external-secrets.io/v1beta1
kind: ExternalSecret
metadata:
  name: database-credentials
  namespace: my-app
spec:
  refreshInterval: "1h"
  secretStoreRef:
    name: kubunity-cluster-secret-store
    kind: ClusterSecretStore
  target:
    name: db-secret # Nom du Secret K8s créé dans le namespace
  data:
    - secretKey: password
      remoteRef:
        key: "production/db/password"
```
