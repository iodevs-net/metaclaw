# Metaclaw Local Setup — Gotchas y Configuración Correcta

## ⚠️ GOTCHA: Provider de MiniMax

**INCORRECTO (NO funciona):**
```toml
default_provider = "anthropic"
default_model = "MiniMax-M2.7-highspeed"
```

**CORRECTO:**
```toml
default_provider = "anthropic-custom:https://api.minimax.io/anthropic"
default_model = "MiniMax-M2.7-highspeed"
```

El provider `anthropic` es el provider oficial de Anthropic y busca `ANTHROPIC_API_KEY`. Solo `anthropic-custom:https://api.minimax.io/anthropic` apunta al endpoint correcto de MiniMax.

---

## 📋 Configuración Completa

### 1. config.toml

```toml
# ZeroClaw Configuración metaclaw-dev

# ⚠️ IMPORTANTE: Provider correcto para MiniMax
default_provider = "anthropic-custom:https://api.minimax.io/anthropic"
default_model = "MiniMax-M2.7-highspeed"
api_key = "TU_MINIMAX_API_KEY"
default_temperature = 0.3
provider_timeout_secs = 300

[gateway]
port = 42617
host = "0.0.0.0"
require_pairing = true

[mcp]
enabled = false
deferred_loading = true
servers = []

[agent]
compact_context = true
max_tool_iterations = 10
max_history_messages = 50

[autonomy]
level = "supervised"
```

### 2. docker-compose.dev.yml

```yaml
services:
  metaclaw:
    image: metaclaw:local
    container_name: metaclaw-dev
    ports:
      - "42777:42617"
    volumes:
      - ./zeroclaw-data:/zeroclaw-data
    environment:
      - ZEROCLAW_CONFIG=/zeroclaw-data/.zeroclaw/config.toml
    restart: unless-stopped
```

---

## 🚀 Levantar Entorno Local

```bash
# 1. Construir imagen
docker build -t metaclaw:local --file Dockerfile .

# 2. Crear estructura de directorios
mkdir -p zeroclaw-data/workspace
mkdir -p zeroclaw-data/.zeroclaw

# 3. Copiar config (con provider correcto)
cp docs/getting-started/METACLAV_LOCAL_SETUP.md zeroclaw-data/.zeroclaw/config.toml
# ⚠️ Editar: cambiar api_key y api_url

# 4. Levantar contenedor
docker compose -f docker-compose.dev.yml up -d

# 5. Obtener pairing code
docker logs metaclaw-dev
# Buscar código como: │ 123456 │

# 6. Pairing
curl -X POST http://localhost:42777/pair \
  -H "X-Pairing-Code: 123456"

# 7. Probar
curl -X POST http://localhost:42777/webhook \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer TU_TOKEN" \
  -d '{"message": "Di solo: Hola"}'
```

---

## 🔑 Variables de Entorno (opcional)

Si prefieres usar variables en lugar de hardcodear en config:

```bash
# .env
MINIMAX_API_KEY=sk-cp-tu-api-key-aqui
```

---

## 📝 Notas Importantes

1. **Modelo correcto:** `MiniMax-M2.7-highspeed` (no `MiniMax-M2.7`)
2. **Endpoint:** `https://api.minimax.io/anthropic` (no `/v1`)
3. **Auth:** MiniMax usa `x-api-key` header (como Anthropic)
4. **Temperatura recomendada:** 0.3 para tareas de coding

---

## 🐛 Errores Comunes

| Error | Causa | Solución |
|--------|--------|----------|
| `OpenRouter API key not set` | Provider wrong | Usar `anthropic-custom:https://...` |
| `401 Unauthorized - invalid x-api-key` | API key wrong | Usar Token Plan API key de MiniMax |
| `Unknown config key ignored: "provider"` | Config mal estructurada | Las claves van en raíz, no en `[provider]` |

---

## 🔗 Referencias

- Docs: https://platform.minimax.io/docs/llms.txt
- Modelos: https://platform.minimax.io/user-center/basic-information/interface-key

---

## Cloud Storage MCPs

Para que el agente pueda crear archivos en la nube (Google Drive, Dropbox, OneDrive):

1. **Obtener credenciales** — ver `docs/getting-started/MCP_CLOUD_STORAGE_SETUP.md`
2. **Agregar al .env** — editar las variables correspondientes
3. **Activar MCPs** — ejecutar `./scripts/activate-mcp-cloud.sh`
4. **Reiniciar** — `docker compose -f docker-compose.dev.yml restart`
