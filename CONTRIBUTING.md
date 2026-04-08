# Contributing to MetaClaw

> **Acerca de MetaClaw**
>
> MetaClaw es un fork personal de [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw), mantenido localmente para uso personal/interno.
>
> - **Upstream**: `/home/leonardo/dev/proyectos/zeroclaw`
> - **Propósito**: Uso personal con modificaciones específicas
>
> Este documento está adaptado de la guía de contribución de ZeroClaw para uso interno.

---

Gracias por tu interés en MetaClaw. Esta guía ha sido adaptada de ZeroClaw para el mantenimiento local de este fork.

---

## ⚠️ Aviso Importante (Fork Personal)

**Este repositorio es un fork personal de ZeroClaw para uso local.**

- No es un proyecto oficial de ZeroClaw Labs
- Las contribuciones están orientadas a necesidades personales
- Para contribuir al proyecto ZeroClaw principal, visita [zeroclaw-labs/zeroclaw](https://github.com/zeroclaw-labs/zeroclaw)

---

## Branch Model

> **`master`** es la rama principal de este fork.

## Development Setup

```bash
# Clone el repositorio (o usa el upstream local)
git clone https://github.com/zeroclaw-labs/zeroclaw.git
cd zeroclaw

# Habilitar el hook pre-push (ejecuta fmt, clippy, tests antes de cada push)
git config core.hooksPath .githooks

# Build
cargo build

# Run tests (todos deben pasar)
cargo test --locked

# Format & lint
./scripts/ci/rust_quality_gate.sh

# Release build
cargo build --release --locked
```

### Pre-push hook

El repo incluye un pre-push hook en `.githooks/` que ejecuta `./scripts/ci/rust_quality_gate.sh` y `cargo test --locked` antes de cada push. Habilítalo con `git config core.hooksPath .githooks`.

## Local Secret Management (Required)

MetaClaw soporta gestión de secretos en capas para desarrollo local e higiene en CI.

### Secret Storage Options

1. **Environment variables** (recomendado para desarrollo local)
   - Copia `.env.example` a `.env` y llena los valores
   - Los archivos `.env` están en .gitignore y deben mantenerse locales
   - Mejor para API keys temporales/locales

2. **Config file** (`~/.metaclaw/config.toml`)
   - Configuración persistente para uso a largo plazo
   - Cuando `secrets.encrypt = true` (default), los valores secretos se encriptan antes de guardar
   - La clave secreta se almacena en `~/.metaclaw/.secret_key` con permisos restringidos
   - Usa `metaclaw onboard` para configuración guiada

### Runtime Resolution Rules

La resolución de API key sigue este orden:

1. Key explícita pasada desde config/CLI
2. Env vars específicas del provider (`OPENROUTER_API_KEY`, `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, ...)
3. Env vars genéricas (`METACLAW_API_KEY`, `API_KEY`)

### Pre-Commit Secret Hygiene (Mandatory)

Antes de cada commit, verifica:

- [ ] No hay archivos `.env` staged (solo `.env.example`)
- [ ] No hay API keys/tokens en código, tests, fixtures, examples, logs o commit messages
- [ ] No hay credenciales en debug output o error payloads
- [ ] `git diff --cached` no tiene strings accidentales tipo secrets

## Architecture: Trait-Based Pluggability

La arquitectura de MetaClaw está construida sobre **traits** — cada subsistema es intercambiable. Esto significa que contribuir una nueva integración es tan simple como implementar un trait y registrarlo en la función factory.

```
src/
├── providers/       # LLM backends     → Provider trait
├── channels/        # Messaging         → Channel trait
├── observability/   # Metrics/logging   → Observer trait
├── runtime/         # Platform adapters → RuntimeAdapter trait
├── tools/           # Agent tools       → Tool trait
├── memory/          # Persistence/brain → Memory trait
└── security/        # Sandboxing        → SecurityPolicy
```

## Code Naming Conventions (Required)

Usa estos defaults a menos que un patrón existente del subsistema claramente los sobreescriba.

- **Rust casing**: modules/files `snake_case`, types/traits/enums `PascalCase`, functions/variables `snake_case`, constants `SCREAMING_SNAKE_CASE`.
- **Domain-first naming**: prefiera nombres de rol explícitos como `DiscordChannel`, `SecurityPolicy`, `SqliteMemory` sobre nombres ambiguos (`Manager`, `Util`, `Helper`).
- **Trait implementers**: mantenga sufijos predecibles (`*Provider`, `*Channel`, `*Tool`, `*Memory`, `*Observer`, `*RuntimeAdapter`).
- **Factory keys**: mantenga minúsculas y estables (`openai`, `discord`, `shell`); evite agregar aliases sin necesidad de migración.
- **Tests**: use nombres orientados a comportamiento (`subject_expected_behavior`) y fixtures neutrales scoped al proyecto.
- **Identity-like labels**: si es unavoidable, use identificadores nativos de ZeroClaw/MetaClaw únicamente (`MetaClawAgent`, `metaclaw_user`).

## Pull Request Checklist

- [ ] Los comandos básicos pasan: `cargo fmt && cargo clippy && cargo test`
- [ ] No hay datos personales/sensibles en código/docs/tests/fixtures/logs/examples/commit messages
- [ ] Los nombres de tests/messages/fixtures/examples son neutrales y focused al proyecto
- [ ] Cualquier texto tipo identity usa labels nativos de ZeroClaw/MetaClaw únicamente

## Commit Convention

Usamos [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add Anthropic provider
feat(provider): add Anthropic provider
fix: path traversal edge case with symlinks
docs: update contributing guide
test: add heartbeat unicode parsing tests
refactor: extract common security checks
chore: bump tokio to 1.43
```

## Code Style

- **Minimal dependencies** — cada crate agrega al binary size
- **Inline tests** — `#[cfg(test)] mod tests {}` al final de cada archivo
- **Trait-first** — define el trait, luego implementa
- **Security by default** — sandbox todo, allowlist, nunca blocklist
- **No unwrap in production code** — usa `?`, `anyhow`, o `thiserror`

## Reporting Issues

- **Bugs**: Incluye OS, Rust version, steps to reproduce, expected vs actual
- **Features**: Describe el caso de uso, propone qué trait extender
- **Security**: Ver [SECURITY.md](SECURITY.md) para responsible disclosure
- **Privacy**: Redacta/anonimiza todos los datos personales e identificadores sensibles antes de postear logs/payloads

## License

Al contribuir, aceptas que tus contribuciones serán licenciadas bajo MIT License.
