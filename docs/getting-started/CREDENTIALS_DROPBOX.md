# Cómo obtener credencial de Dropbox MCP

## Paso 1: Crear Dropbox App

1. Ve a [dropbox.com/developers/apps](https://www.dropbox.com/developers/apps)
2. Click **"Create app"**
3. **Choose an API**: Scoped access
4. **Choose the type of access**: Full Dropbox
5. **Name**: `metaclaw-dropbox`
6. Click **Create app**

## Paso 2: Configurar permisos

1. En tu app → **Permissions** tab
2. Habilitar estos permisos:
   - `files.content.write`
   - `files.metadata.write`
   - `sharing.write`
3. Click **Submit**
4. Click **Apply**

## Paso 3: Generar Access Token

### Opción A: OAuth2 (más seguro)

1. En tu app → **Settings** tab
2. Buscar **"OAuth 2"**
3. En **"Generated access token"**, click **"Generate"**
4. **Guardar el token** (es largo, empieza con `sl.`)

### Opción B: Via curl

```bash
# Reemplaza con tu app key y secret
APP_KEY=tu_app_key
APP_SECRET=tu_app_secret

# 1. Obtener code
curl -X POST https://api.dropboxapi.com/oauth2/token \
  -d "code=AUTH_CODE&grant_type=authorization_code&redirect_uri=https://localhost" \
  -u "$APP_KEY:$APP_SECRET"

# 2. Refresh token (para long-lived)
curl -X POST https://api.dropboxapi.com/oauth2/token \
  -d "refresh_token=REFRESH_TOKEN&grant_type=refresh_token&client_id=$APP_KEY&client_secret=$APP_SECRET"
```

## Paso 4: Agregar al .env

```bash
DROPBOX_ACCESS_TOKEN=sl.XXXXXx...tu-token-completo
```

## Nota importante

Los tokens de Dropbox **pueden expirar**. Si el MCP deja de funcionar, regenera el token.

## Permisos necesarios

- `files.content.write` - Para subir archivos
- `files.metadata.write` - Para crear carpetas
- `sharing.write` - Para generar links compartidos
