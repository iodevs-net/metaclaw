#!/bin/bash
# Script para activar MCPs de cloud storage
# Uso: ./scripts/activate-mcp-cloud.sh

set -e

CONFIG_FILE="zeroclaw-data/.zeroclaw/config.toml"

echo "Activando MCPs de cloud storage..."

# Verificar que las variables están configuradas
if [ -z "$GOOGLE_CLIENT_ID" ] && [ -z "$DROPBOX_ACCESS_TOKEN" ] && [ -z "$MICROSOFT_CLIENT_ID" ]; then
    echo "⚠️  No hay credenciales configuradas."
    echo "   Edita .env y agrega las credenciales primero."
    exit 1
fi

# Backup config actual
cp "$CONFIG_FILE" "$CONFIG_FILE.bak"

# Crear nueva config con MCPs activos
cat > "$CONFIG_FILE" << 'CONFIG'
# ZeroClaw Configuración metaclaw-dev

default_provider = "anthropic-custom:https://api.minimax.io/anthropic"
default_model = "MiniMax-M2.7-highspeed"
api_key = "${MINIMAX_API_KEY}"
default_temperature = 0.3
provider_timeout_secs = 300

[mcp]
enabled = true
deferred_loading = true

CONFIG

# Agregar Google Drive si está configurado
if [ -n "$GOOGLE_CLIENT_ID" ]; then
    echo '[[mcp.servers]]' >> "$CONFIG_FILE"
    echo 'name = "google-drive"' >> "$CONFIG_FILE"
    echo 'transport = "stdio"' >> "$CONFIG_FILE"
    echo 'command = "npx"' >> "$CONFIG_FILE"
    echo 'args = ["-y", "@modelcontextprotocol/server-gdrive"]' >> "$CONFIG_FILE"
    echo 'env = {' >> "$CONFIG_FILE"
    echo '  GOOGLE_CLIENT_ID = "${GOOGLE_CLIENT_ID}"' >> "$CONFIG_FILE"
    echo '  GOOGLE_CLIENT_SECRET = "${GOOGLE_CLIENT_SECRET}"' >> "$CONFIG_FILE"
    echo '  GOOGLE_REFRESH_TOKEN = "${GOOGLE_REFRESH_TOKEN}"' >> "$CONFIG_FILE"
    echo '}' >> "$CONFIG_FILE"
    echo "✅ Google Drive MCP añadido"
fi

# Agregar Dropbox si está configurado
if [ -n "$DROPBOX_ACCESS_TOKEN" ]; then
    echo '[[mcp.servers]]' >> "$CONFIG_FILE"
    echo 'name = "dropbox"' >> "$CONFIG_FILE"
    echo 'transport = "stdio"' >> "$CONFIG_FILE"
    echo 'command = "npx"' >> "$CONFIG_FILE"
    echo 'args = ["-y", "@microagents/mcp-server-dropbox"]' >> "$CONFIG_FILE"
    echo 'env = {' >> "$CONFIG_FILE"
    echo '  DROPBOX_ACCESS_TOKEN = "${DROPBOX_ACCESS_TOKEN}"' >> "$CONFIG_FILE"
    echo '}' >> "$CONFIG_FILE"
    echo "✅ Dropbox MCP añadido"
fi

# Agregar OneDrive si está configurado
if [ -n "$MICROSOFT_CLIENT_ID" ]; then
    echo '[[mcp.servers]]' >> "$CONFIG_FILE"
    echo 'name = "onedrive"' >> "$CONFIG_FILE"
    echo 'transport = "stdio"' >> "$CONFIG_FILE"
    echo 'command = "npx"' >> "$CONFIG_FILE"
    echo 'args = ["-y", "@softeria/ms-365-mcp-server"]' >> "$CONFIG_FILE"
    echo 'env = {' >> "$CONFIG_FILE"
    echo '  MICROSOFT_CLIENT_ID = "${MICROSOFT_CLIENT_ID}"' >> "$CONFIG_FILE"
    echo '  MICROSOFT_TENANT_ID = "common"' >> "$CONFIG_FILE"
    echo '}' >> "$CONFIG_FILE"
    echo "✅ OneDrive MCP añadido"
fi

# Agregar resto de config
cat >> "$CONFIG_FILE" << 'CONFIG'

[gateway]
port = 42617
host = "0.0.0.0"
require_pairing = true

[agent]
compact_context = true
max_tool_iterations = 10
max_history_messages = 50

[autonomy]
level = "supervised"
CONFIG

echo ""
echo "✅ MCPs activados en $CONFIG_FILE"
echo ""
echo "Reinicia metaclaw con:"
echo "  docker compose -f docker-compose.dev.yml restart"
echo ""
