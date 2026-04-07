# MCP Cloud Storage Setup — Google Drive, Dropbox, OneDrive

Este documento configura metaclaw para usar MCP servers que permiten crear archivos en la nube.

---

## Guía rápida por servicio

| Servicio | Guía | Tiempo |
|----------|-------|--------|
| Google Drive | [CREDENTIALS_GOOGLE_DRIVE.md](./CREDENTIALS_GOOGLE_DRIVE.md) | ~10 min |
| Dropbox | [CREDENTIALS_DROPBOX.md](./CREDENTIALS_DROPBOX.md) | ~5 min |
| OneDrive | [CREDENTIALS_ONEDRIVE.md](./CREDENTIALS_ONEDRIVE.md) | ~10 min |

---

## Configuración en metaclaw

### 1. Editar .env

```bash
# Google Drive
GOOGLE_CLIENT_ID=tu-client-id.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=tu-client-secret
GOOGLE_REFRESH_TOKEN=1//tu-refresh-token

# Dropbox
DROPBOX_ACCESS_TOKEN=sl.XXXXXx...tu-token

# OneDrive
MICROSOFT_CLIENT_ID=tu-client-id
MICROSOFT_TENANT_ID=common
```

### 2. Descomentar MCPs en config.toml

Editar `zeroclaw-data/.zeroclaw/config.toml` y descomentar los servers MCP deseados:

```toml
[mcp]
enabled = true
deferred_loading = true

[[mcp.servers]]
name = "google-drive"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-gdrive"]
env = {
  GOOGLE_CLIENT_ID = "${GOOGLE_CLIENT_ID}"
  GOOGLE_CLIENT_SECRET = "${GOOGLE_CLIENT_SECRET}"
  GOOGLE_REFRESH_TOKEN = "${GOOGLE_REFRESH_TOKEN}"
}
```

### 3. Reiniciar metaclaw

```bash
docker compose -f docker-compose.dev.yml restart
```

---

## Herramientas disponibles

### Google Drive (`gdrive__*`)
- `list_files` - Listar archivos
- `upload_file` - Subir archivo
- `create_folder` - Crear carpeta
- `share_file` - Generar link compartible

### Dropbox (`dropbox__*`)
- `list_files` - Listar archivos
- `upload_file` - Subir archivo
- `create_folder` - Crear carpeta
- `get_shareable_link` - Generar link

### OneDrive (`onedrive__*`)
- `list_files` - Listar archivos
- `upload_file` - Subir archivo
- `create_folder` - Crear carpeta
- `share_file` - Generar link compartible

---

## Notas importantes

1. **Links compartidos** son públicos por defecto
2. **Tokens pueden expirar** - regenerate si MCP deja de funcionar
3. **OneDrive usa device code flow** - la primera vez te pide login en navegador
