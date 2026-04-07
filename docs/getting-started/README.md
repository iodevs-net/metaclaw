# Getting Started — Metaclaw Local

## Contenido

### Configuración inicial
- [METACLAV_LOCAL_SETUP.md](./METACLAV_LOCAL_SETUP.md) — Setup completo del entorno

### MCP Cloud Storage (archivos en la nube)
- [MCP_CLOUD_STORAGE_SETUP.md](./MCP_CLOUD_STORAGE_SETUP.md) — Resumen de configuración
- [CREDENTIALS_GOOGLE_DRIVE.md](./CREDENTIALS_GOOGLE_DRIVE.md) — Cómo obtener credenciales Google
- [CREDENTIALS_DROPBOX.md](./CREDENTIALS_DROPBOX.md) — Cómo obtener credenciales Dropbox
- [CREDENTIALS_ONEDRIVE.md](./CREDENTIALS_ONEDRIVE.md) — Cómo obtener credenciales OneDrive

## Quick Start

```bash
# 1. Setup interactivo
./scripts/setup-cloud-mcp.sh

# 2. Seguir las guías de credenciales

# 3. Activar MCPs
./scripts/activate-mcp-cloud.sh

# 4. Reiniciar
docker compose -f docker-compose.dev.yml restart
```

## Estructura de archivos

```
docs/getting-started/
├── METACLAV_LOCAL_SETUP.md    # Setup inicial
├── MCP_CLOUD_STORAGE_SETUP.md # Configuración MCP
├── CREDENTIALS_*.md            # Guías para cada servicio
└── README.md                   # Este archivo
```
