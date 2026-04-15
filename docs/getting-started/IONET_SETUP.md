# IONet MCP Integration Setup

Configura MetaClaw para conectar con los sistemas de IONet.

---

## Índice

1. [iOdesk MCP Server](#iodesk-mcp-server)
2. [Microsoft 365 / Intune](#microsoft-365--intune)
3. [Email (Outlook/M365)](#email-outlookm365)

---

## iOdesk MCP Server

El MCP de iOdesk permite acceder al helpdesk de IONet para consultar tickets, clientes, dispositivos y más.

### Requisitos

1. **Levantar el contenedor MCP de iOdesk:**
   ```bash
   cd /home/leonardo/dev/proyectos/iodesk-3
   TRANSPORT=http docker compose --profile dev up -d mcp
   ```

2. **Verificar que funciona:**
   ```bash
   curl http://localhost:8765/health
   # Respuesta: {"status": "ok", "service": "iodesk-mcp", "tools_count": 20}
   ```

### Configuración en MetaClaw

Editar `zeroclaw-data/.zeroclaw/config.toml`:

```toml
[mcp]
enabled = true
deferred_loading = true

[[mcp.servers]]
name = "iodesk"
transport = "http"
url = "http://localhost:8765/sse"
tool_timeout_secs = 60
```

### Herramientas disponibles (20+)

| Herramienta | Descripción |
|------------|-------------|
| `iodesk__ticket_list` | Lista tickets con filtros |
| `iodesk__ticket_get` | Detalle de un ticket |
| `iodesk__ticket_create` | Crear nuevo ticket |
| `iodesk__ticket_update` | Actualizar estado |
| `iodesk__cliente_list` | Lista clientes |
| `iodesk__cliente_get` | Detalle de cliente |
| `iodesk__cliente_dispositivos` | Dispositivos de cliente |
| `iodesk__tecnico_stats` | Estadísticas de técnico |
| `iodesk__dispositivo_list` | Lista dispositivos |
| `iodesk__dispositivo_por_serial` | Buscar por serial |
| `iodesk__dashboard` | Resumen general |
| `iodesk__estadisticas_mensuales` | Stats mensuales |
| `iodesk__informes_tecnicos` | Horas por técnico |
| `iodesk__sesion_iniciar` | Iniciar sesión trabajo |
| `iodesk__sesion_terminar` | Terminar sesión |
| `iodesk__bitacora_agregar` | Agregar nota |

### Ejemplos de uso

```
"¿Cuántos tickets abiertos tiene ACME Spa?"

"Ver estadísticas de Leonardo de este mes"

"Crear ticket urgente para Restaurant X - servidor no responde"

"Buscar notebook Dell serial SN-2024-00142"

"¿Cuántas horas trabajó cada técnico en marzo?"
```

---

## Microsoft 365 / Intune

Configura el tool de Microsoft 365 para acceder a Intune (gestión de dispositivos).

### Azure AD App Registration

1. Ve a [Azure Portal](https://portal.azure.com) → Azure Active Directory → App registrations
2. Crea nuevo registro:
   - Name: `MetaClaw-IONet`
   - Supported account types: Single tenant
3. En **API permissions** agrega:
   - Microsoft Graph → Delegated permissions:
     - `Mail.Read`, `Mail.Send`
     - `Calendars.ReadWrite`
     - `Team.ReadBasic.All`
   - Microsoft Graph → Application permissions:
     - `DeviceManagementManagedDevices.Read.All`
     - `DeviceManagementManagedDevices.ReadWrite.All`
     - `DeviceManagementApps.Read.All`
4. Genera **Client secret** en Certificates & secrets

### Configuración

```toml
[tools.microsoft365]
enabled = true
auth_flow = "client_credentials"
tenant_id = "tu-tenant-id-de-azure"
client_id = "tu-app-id"
client_secret = "tu-secret"
user_id = "agente@ionet.cl"
```

### Herramientas Intune (16+)

| Herramienta | Descripción |
|------------|-------------|
| `microsoft365__intune_device_list` | Listar dispositivos |
| `microsoft365__intune_device_get` | Detalle dispositivo |
| `microsoft365__intune_device_by_user` | Dispositivos de usuario |
| `microsoft365__intune_device_wipe` | Wipe remoto ⚠️ |
| `microsoft365__intune_device_retire` | Retire dispositivo |
| `microsoft365__intune_device_disable` | Bloquear acceso |
| `microsoft365__intune_compliance_summary` | Resumen compliance |
| `microsoft365__intune_apps_list` | Lista apps móviles |

### Ejemplos de uso

```
"¿Cuántos equipos Windows tenemos en Intune?"

"Estado del notebook de Juan"

"Wipe remoto del equipo SN-123 robado"

"Informe de cumplimiento de dispositivos"
```

---

## Email (Outlook/M365)

Configura el canal de email para recibir tareas por correo.

### App Password en M365

1. Ve a Azure AD → Security → Authentication methods → App passwords
2. Genera una App Password para MetaClaw

### Configuración

```toml
[channels.email]
enabled = true
imap_host = "outlook.office365.com"
imap_port = 993
smtp_host = "smtp.office365.com"
smtp_port = 587
smtp_tls = true
username = "agente@ionet.cl"
password = "xxxx-xxxx-xxxx-xxxx"  # App Password
from_address = "agente@ionet.cl"
allowed_senders = ["*@ionet.cl"]
```

---

## Resumen de integración IONet

```
┌─────────────────────────────────────────────────────────────┐
│                    MetaClaw (IONet Agent)                   │
│                                                             │
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────────┐  │
│  │   Email     │  │   iOdesk     │  │   Microsoft 365  │  │
│  │  Channel    │  │  MCP Server   │  │   Tool          │  │
│  │             │  │              │  │                  │  │
│  │ Recibir     │  │ Tickets      │  │ Mail, Teams     │  │
│  │ tareas      │  │ Clientes     │  │ Calendar        │  │
│  │ por email   │  │ Dispositivos │  │ Intune (MDM)    │  │
│  │             │  │ Stats        │  │ OneDrive        │  │
│  └─────────────┘  └──────────────┘  └─────────────────┘  │
│         │                 │                    │          │
└─────────┼─────────────────┼────────────────────┼──────────┘
          │                 │                    │
          ▼                 ▼                    ▼
     outlook.office   localhost:8765       graph.microsoft.com
```

---

## Verificación

Después de configurar, reinicia MetaClaw:

```bash
docker compose -f docker-compose.dev.yml restart
```

Verifica que el MCP está conectado:

```bash
zeroclaw status
# Debe mostrar "iodesk" en MCP servers
```
