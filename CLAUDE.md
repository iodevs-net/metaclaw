# CLAUDE.md — MetaClaw (Fork de ZeroClaw)

> **Acerca de MetaClaw**
>
> MetaClaw es un fork personal de [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw), mantenido localmente para uso personal/interno.
>
> - **Upstream**: `/home/leonardo/dev/proyectos/zeroclaw`
> - **Propósito**: Uso personal con modificaciones específicas
>
> Las instrucciones compartidas viven en [`AGENTS.md`](./AGENTS.md).

## Build & Test Speed

**CRITICAL: Default `cargo build`/`cargo test` uses slow `release` profile (fat LTO, codegen-units=1, ~8-9 min).**

For fast iteration:

- `cargo build` / `cargo test` → dev profile (opt-level=1, segundos)
- `cargo build --profile release-fast` → fast release build (~4 min)
- `cargo test --profile ci` → fast tests (~2 min)

Never run `cargo build --release` for development.

## Claude Code Settings

Claude Code should read and follow all instructions in `AGENTS.md` at the repository root for project conventions, commands, risk tiers, workflow rules, and anti-patterns.

> **Nota**: Este es un fork personal de ZeroClaw. Cualquier referencia a "ZeroClaw" en la documentación interna se refiere al proyecto upstream.

## Hooks

_No custom hooks defined yet._

## Slash Commands

_No custom slash commands defined yet._

## Metaclaw Local Setup

⚠️ **GOTCHA: Configuración de MiniMax**

**Provider correcto para MiniMax-M2.7:**
```toml
default_provider = "anthropic-custom:https://api.minimax.io/anthropic"
default_model = "MiniMax-M2.7-highspeed"
```

Ver documento completo: `docs/getting-started/METACLAV_LOCAL_SETUP.md`
