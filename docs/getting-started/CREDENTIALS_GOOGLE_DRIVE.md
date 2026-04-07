# Cómo obtener credenciales de Google Drive MCP

## Paso 1: Crear proyecto en Google Cloud

1. Ve a [console.cloud.google.com](https://console.cloud.google.com)
2. Click en **"Select a project"** → **"New project"**
3. Nombre: `metaclaw-gdrive`
4. Click **Create**

## Paso 2: Habilitar Google Drive API

1. En el menú lateral → **APIs & Services** → **Library**
2. Buscar **"Google Drive API"**
3. Click → **Enable**

## Paso 3: Crear credenciales OAuth

1. Ve a **APIs & Services** → **Credentials**
2. Click **+ Create Credentials** → **OAuth client ID**
3. Application type: **Desktop app**
4. Nombre: `metaclaw-gdrive`
5. Click **Create**
6. **Guarda el Client ID y Client Secret** (aparecen en popup)

## Paso 4: Generar Refresh Token

Necesitas un refresh token permanente. Hay varias formas:

### Opción A: Script de Python (recomendado)

```python
# Instalar: pip install google-auth google-api-python-client

import os
from google_auth_oauthlib.flow import InstalledAppFlow

SCOPES = ['https://www.googleapis.com/auth/drive.file']

flow = InstalledAppFlow.from_client_secrets_file(
    'client_secrets.json',  # Descarga el JSON de las credenciales
    SCOPES
)
creds = flow.run_local_server(port=0)

# Guardar refresh token
print(f"Refresh token: {creds.token}")
print(f"Client ID: {creds.client_id}")
print(f"Client Secret: {creds.client_secret}")
```

### Opción B: Con gcloud CLI

```bash
# Instalar gcloud: https://cloud.google.com/sdk/docs/install

gcloud auth login
gcloud auth print-refresh-token
```

## Paso 5: Agregar al .env

```bash
GOOGLE_CLIENT_ID=tu-client-id.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=tu-client-secret
GOOGLE_REFRESH_TOKEN=1//tu-refresh-token
```

## Nota importante

El refresh token de OAuth es **permanente** (no expira) a menos que revoques acceso.

## Troubleshooting

- Si dice "This app isn't verified", ve a Seguridad y activa "Modo avanzado"
- El refresh token funciona mientras la app esté habilitada
