# MetaClaw — Fork de ZeroClaw

> **Acerca de MetaClaw**
>
> MetaClaw es un fork personal de [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw), mantenido localmente para uso personal/interno.
>
> - **Upstream**: Fork de ZeroClaw (repositorio local en `/home/leonardo/dev/proyectos/zeroclaw`)
> - **Propósito**: Uso personal con modificaciones específicas
> - **Compatibilidad**: Mantiene compatibilidad con la API y configuración de ZeroClaw

---

## Tabla de Contenidos

1. [Relación con ZeroClaw](#relación-con-zeroclaw)
2. [Diferencias Técnicas](#diferencias-técnicas)
3. [Compatibilidad](#compatibilidad)
4. [Auditoría de Seguridad](#auditoría-de-seguridad)
5. [Recomendaciones](#recomendaciones)
6. [Mantenimiento](#mantenimiento)

---

## Relación con ZeroClaw

### Upstream

MetaClaw es un fork del repositorio [zeroclaw-labs/zeroclaw](https://github.com/zeroclaw-labs/zeroclaw). El repositorio upstream está disponible localmente en:

```
/home/leonardo/dev/proyectos/zeroclaw
```

### Historia

- **Fecha del fork**: No documentada (repo personal)
- **Propósito**: Uso personal/interno con modificaciones específicas
- **Mantenimiento**: Sincronización manual con upstream según necesidad

---

## Diferencias Técnicas

### 1. Identidad del Paquete

| Aspecto | ZeroClaw | MetaClaw |
|---------|----------|----------|
| Nombre del crate | `zeroclawlabs` | `metaclaw` |
| Nombre en crates.io | `zeroclawlabs` | N/A (no publicado) |
| Binario principal | `zeroclaw` | `zeroclaw` (compatible) |
| Biblioteca | `zeroclaw` | `zeroclaw` (compatible) |
| Autor | ZeroClaw Labs | `theonlyhennygod` |
| Versión inicial | 0.6.8 | 0.6.8 (heredada) |

### 2. Archivos Modificados

#### Documentación Actualizada

| Archivo | Cambio |
|---------|--------|
| `README.md` | Branding actualizado a MetaClaw, nota de fork |
| `CLAUDE.md` | Contexto de fork añadido |
| `AGENTS.md` | Información de fork añadida |
| `CONTRIBUTING.md` | Guía adaptada para uso personal |
| `CHANGELOG.md` | Nota de fork añadida |
| `CODE_OF_CONDUCT.md` | Nota de fork añadida |
| `Cargo.toml` | Nombre de paquete: `metaclaw` |
| `SECURITY.md` | Nota de heredación de ZeroClaw |
| `NOTICE` | Attribution completa a ZeroClaw Labs |
| `install.sh` | Header actualizado |
| `setup.bat` | Título actualizado |
| `docs/README.md` | Actualizado a español, nota de fork |
| `docs/SUMMARY.md` | Tabla de contenidos simplificada |
| `docs/i18n/README.md` | Índice reducido a español |
| `docs/i18n/es/README.md` | Nota de fork añadida |

#### Archivos No Modificados (Compatibilidad)

Los siguientes archivos mantienen referencias a ZeroClaw por compatibilidad técnica:

- `src/` — Código fuente (mantiene `ZEROCLAW_*` env vars)
- `scripts/` — Scripts de deployment
- `tests/` — Suite de tests
- `.env.example` — Variables `ZEROCLAW_*`

### 3. Cambios de Branding vs Funcionalidad

| Categoría | Cambio | Razón |
|-----------|--------|-------|
| **Branding** | ✅ Actualizado | Distinguir de ZeroClaw oficial |
| **API/CLI** | ❌ Sin cambios | Compatibilidad con herramientas existentes |
| **Variables de entorno** | ❌ Sin cambios | Compatibilidad con configuraciones existentes |
| **Estructura de directorios** | ❌ Sin cambios | `~/.zeroclaw/` mantenido |
| **Protocolo de red** | ❌ Sin cambios | Compatibilidad con gateway/webhooks |

---

## Compatibilidad

### Variables de Entorno `ZEROCLAW_*` (100% Compatible)

MetaClaw mantiene **todas** las variables de entorno de ZeroClaw:

```bash
# API y Proveedores
ZEROCLAW_API_KEY           # Clave API genérica
ZEROCLAW_PROVIDER         # Proveedor por defecto
ZEROCLAW_MODEL            # Modelo por defecto
ZEROCLAW_PROVIDER_URL     # URL personalizada para Ollama
ZEROCLAW_EXTRA_HEADERS    # Headers HTTP adicionales

# Workspace y Configuración
ZEROCLAW_WORKSPACE        # Directorio de workspace
ZEROCLAW_CONFIG_DIR       # Directorio de configuración
ZEROCLAW_LOCALE          # Idioma (en, es, etc.)

# Gateway
ZEROCLAW_GATEWAY_PORT    # Puerto del gateway
ZEROCLAW_GATEWAY_HOST    # Host del gateway
ZEROCLAW_GATEWAY_TIMEOUT_SECS

# Seguridad
ZEROCLAW_AUDIT_SIGNING_KEY  # Clave HMAC para logs de auditoría

# Canales
ZEROCLAW_WHATSAPP_APP_SECRET
ZEROCLAW_LINQ_SIGNING_SECRET
ZEROCLAW_NEXTCLOUD_TALK_WEBHOOK_SECRET

# Proxy
ZEROCLAW_PROXY_ENABLED
ZEROCLAW_HTTP_PROXY
ZEROCLAW_HTTPS_PROXY
ZEROCLAW_ALL_PROXY
ZEROCLAW_NO_PROXY
ZEROCLAW_PROXY_SCOPE
ZEROCLAW_PROXY_SERVICES

# Almacenamiento
ZEROCLAW_STORAGE_PROVIDER
ZEROCLAW_STORAGE_DB_URL
ZEROCLAW_STORAGE_CONNECT_TIMEOUT_SECS

# Skills
ZEROCLAW_OPEN_SKILLS_ENABLED
ZEROCLAW_OPEN_SKILLS_DIR
ZEROCLAW_SKILLS_ALLOW_SCRIPTS
ZEROCLAW_SKILLS_PROMPT_MODE
```

### Estructura de Directorios

```
~/.zeroclaw/           # Compatible con ZeroClaw
├── config.toml        # Configuración principal
├── workspace/         # Workspace del agente
├── .metaclaw/         # ❌ NO creado (mantiene ~/.zeroclaw)
├── auth-profiles.json # Perfiles de autenticación OAuth
├── credentials.db     # Credenciales encriptadas
└── ...
```

### Binarios y Comandos

```bash
# MetaClaw genera el mismo binario que ZeroClaw
zeroclaw --help        # ✅ Funciona igual
zeroclaw onboard       # ✅ Funciona igual
zeroclaw gateway       # ✅ Funciona igual
zeroclaw daemon        # ✅ Funciona igual
```

---

## Auditoría de Seguridad

### Resumen

| Categoría | Estado | Notas |
|-----------|--------|-------|
| Secretos expuestos | ✅ Ninguno | Placeholders apropiados |
| Credenciales de test | ✅ Seguro | Tokens de test son ejemplos |
| Dependencias | ⚠️ 3 ignoradas | Vulnerabilidades via extism/wasmtime |
| Gestión de credenciales | ✅ Correcto | Env vars + config encriptado |
| Políticas de workspace | ✅ Correcto | Aislamiento habilitado |

### Vulnerabilidades Conocidas Ignoradas

| ID | Descripción | Gravedad | Mitigación |
|----|-------------|----------|------------|
| RUSTSEC-2026-0006 | wasmtime segfault via extism | Alta | Feature-gated (plugins) |
| RUSTSEC-2026-0020 | WASI resource exhaustion | Alta | Awaiting extism upgrade |
| RUSTSEC-2026-0021 | WASI http fields panic | Media | Awaiting extism upgrade |

> ⚠️ **Nota**: Estas vulnerabilidades están en la feature `plugins` (Extism/WASM). Si no usas plugins, el riesgo es mínimo.

### Archivos Grandes (>3000 líneas)

| Archivo | Líneas | Recomendación |
|---------|--------|---------------|
| `src/config/schema.rs` | ~14,000 | Considerar modularización |
| `src/onboard/wizard.rs` | ~7,700 | Considerar modularización |
| `src/agent/loop_.rs` | ~5,300 | Considerar modularización |
| `src/security/policy.rs` | ~3,200 | Considerar modularización |

---

## Recomendaciones

### Mantenimiento del Fork

1. **Sincronización con Upstream**
   ```bash
   # Desde el directorio de metaclaw
   git remote add upstream /home/leonardo/dev/proyectos/zeroclaw
   git fetch upstream
   git merge upstream/master
   ```

2. **Resolución de Conflictos**
   - Priorizar cambios de metaclaw en archivos de documentación
   - Priorizar upstream en cambios de seguridad críticos
   - Documentar cualquier modificación funcional

### Seguridad

1. **Monitorear vulnerabilidades**
   ```bash
   # Ejecutar regularmente
   cargo deny check advisories
   cargo deny check licenses
   ```

2. **Actualizar extism cuando parchen estén disponibles**
   - RUSTSEC-2026-0006
   - RUSTSEC-2026-0020
   - RUSTSEC-2026-0021

### Compatibilidad

1. **Antes de cambiar variables de entorno**
   - Verificar que no rompan configuraciones existentes
   - Documentar cambios en este archivo

2. **Antes de cambiar estructura de directorios**
   - Mantener backward compatibility
   - Proporcionar migración de datos si es necesario

---

## Mantenimiento

### Registro de Cambios vs Upstream

| Fecha | Cambio | Razón |
|-------|--------|-------|
| 2026-04-02 | Documentación actualizada | Añadir contexto de fork |
| 2026-04-02 | Cargo.toml renombrado | Identidad de paquete |
| 2026-04-02 | Auditoría de seguridad | Evaluación inicial |

### Verificación de Compatibilidad

Antes de cada merge con upstream:

```bash
# 1. Compilar
cargo build --all-targets

# 2. Ejecutar tests
cargo test

# 3. Verificar linting
cargo clippy --all-targets -- -D warnings

# 4. Verificar formatting
cargo fmt --all -- --check

# 5. Verificar dependencias
cargo deny check
```

---

## Contacto y Soporte

> **Nota**: Este es un proyecto personal de fork. No hay soporte oficial.

- **Upstream**: [zeroclaw-labs/zeroclaw](https://github.com/zeroclaw-labs/zeroclaw)
- **Autor de MetaClaw**: `theonlyhennygod`

---

*Última actualización: 2026-04-02*
