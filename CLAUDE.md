# CLAUDE.md — ZeroClaw (Claude Code)

> **Shared instructions live in [`AGENTS.md`](./AGENTS.md).**
> This file contains only Claude Code-specific directives.

## Build & Test Speed

**CRITICAL: Default `cargo build`/`cargo test` uses slow `release` profile (fat LTO, codegen-units=1, ~8-9 min).**

For fast iteration:
- `cargo build` / `cargo test` → dev profile (opt-level=1, segundos)
- `cargo build --profile release-fast` → fast release build (~4 min)
- `cargo test --profile ci` → fast tests (~2 min)

Never run `cargo build --release` for development.

## Claude Code Settings

Claude Code should read and follow all instructions in `AGENTS.md` at the repository root for project conventions, commands, risk tiers, workflow rules, and anti-patterns.

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
