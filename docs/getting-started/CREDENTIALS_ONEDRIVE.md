# Cómo obtener credenciales de OneDrive/Microsoft 365 MCP

## Paso 1: Registrar app en Azure AD

1. Ve a [portal.azure.com](https://portal.azure.com)
2. Buscar **"Microsoft Entra ID"** (antes Azure AD)
3. En el menú → **App registrations** → **New registration**
4. Nombre: `metaclaw-onedrive`
5. **Supported account types**: Accounts in any organizational directory (Any Microsoft Entra ID - Multitenant)
6. Click **Register**
7. **Guardar el Application (client) ID**

## Paso 2: Crear Client Secret

1. En tu app → **Certificates & secrets**
2. Click **New client secret**
3. Descripción: `metaclaw-secret`
4. Expires: 24 months (o tu preferencia)
5. Click **Add**
6. **Copiar el secret value** (solo aparece una vez)

## Paso 3: Configurar permisos de API

1. En tu app → **API permissions**
2. Click **Add a permission**
3. **Microsoft Graph** → **Delegated permissions**
4. Buscar y agregar:
   - `Files.ReadWrite.All`
   - `Sites.ReadWrite.All`
   - `User.Read`
5. Click **Add permissions**
6. Si aparece warning de "Not verified", hacer clic en **"Grant admin consent"** (necesitas ser admin)

## Paso 4: Obtener Tenant ID

1. En **Microsoft Entra ID** → **Overview**
2. **Copy the Tenant ID**

## Paso 5: Agregar al .env

```bash
MICROSOFT_CLIENT_ID=tu-application-client-id
MICROSOFT_TENANT_ID=tu-tenant-id
```

El servidor `@softeria/ms-365-mcp-server` usa **device code flow**, así que la primera vez que lo ejecutes:
- Te dará un URL y código
- Abres el URL en navegador
- Ingresas el código
- Inicias sesión con tu cuenta Microsoft
- Listo, queda autenticado

## Nota importante

- El Tenant ID `common` funciona para cuentas personales y corporativas
- Para solo cuentas corporativas, usa tu tenant específico
- Los permisos `Files.ReadWrite.All` permiten leer/escribir en todo OneDrive

## Troubleshooting

- Si dice "Need admin approval", necesitas que un admin approve los permisos
- Para cuentas personales de Outlook/Hotmail, `common` funciona bien
