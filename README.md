# MetaClaw — Personal AI Assistant (Fork de ZeroClaw)

> **Acerca de MetaClaw**
>
> MetaClaw es un fork personal del proyecto [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw), mantenido localmente para uso personal/interno.
>
> - **Upstream**: Fork de ZeroClaw (repositorio local en `/home/leonardo/dev/proyectos/zeroclaw`)
> - **Propósito**: Uso personal con modificaciones específicas para necesidades individuales
> - **Compatibilidad**: Mantiene compatibilidad con la API y configuración de ZeroClaw
> - **Estado**: Proyecto activo de mantenimiento propio, no afiliado con ZeroClaw Labs
>
> Para información general sobre ZeroClaw, consulta el [repositorio upstream](https://github.com/zeroclaw-labs/zeroclaw).

---

<p align="center">
  <img src="https://raw.githubusercontent.com/zeroclaw-labs/zeroclaw/master/docs/assets/zeroclaw-banner.png" alt="ZeroClaw" width="600" />
</p>

<h1 align="center">🦀 MetaClaw — Personal AI Assistant</h1>

<p align="center">
  <strong>Zero overhead. Zero compromise. 100% Rust. 100% Agnostic.</strong><br>
  ⚡️ <strong>Runs on $10 hardware with <5MB RAM: That's 99% less memory than OpenClaw and 98% cheaper than a Mac mini!</strong>
</p>

<p align="center">
  <a href="LICENSE-APACHE"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache%202.0-blue.svg" alt="License: MIT OR Apache-2.0" /></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-edition%202024-orange?logo=rust" alt="Rust Edition 2024" /></a>
</p>

> **Nota**: Este es un fork personal de ZeroClaw para uso local. Para el proyecto principal, visite [zeroclaw-labs/zeroclaw](https://github.com/zeroclaw-labs/zeroclaw).

---

MetaClaw es un asistente de IA personal que puedes ejecutar en tus propios dispositivos. Responde en los canales que ya usas (WhatsApp, Telegram, Slack, Discord, Signal, iMessage, Matrix, IRC, Email, Bluesky, Nostr, Mattermost, Nextcloud Talk, DingTalk, Lark, QQ, Reddit, LinkedIn, Twitter, MQTT, WeChat Work, y más). Tiene un dashboard web para control en tiempo real y puede conectarse a periféricos de hardware (ESP32, STM32, Arduino, Raspberry Pi). El Gateway es solo el plano de control — el producto es el asistente.

Si quieres un asistente personal, single-user que se sienta local, rápido y siempre activo, esto es lo que necesitas.

<p align="center">
  <a href="docs/README.md">Docs</a> ·
  <a href="docs/architecture.md">Architecture</a> ·
  <a href="#quick-start">Getting Started</a> ·
  <a href="#migrating-from-openclaw">Migrating from OpenClaw</a> ·
  <a href="docs/ops/troubleshooting.md">Troubleshoot</a>
</p>

> **Preferred setup:** run `metaclaw onboard` in your terminal. MetaClaw Onboard te guía paso a paso a través de la configuración del gateway, workspace, canales y proveedor. Es la ruta de configuración recomendada y funciona en macOS, Linux y Windows (via WSL2). ¿Nueva instalación? Empieza aquí: [Getting started](#quick-start)

### Subscription Auth (OAuth)

- **OpenAI Codex** (ChatGPT subscription)
- **Gemini** (Google OAuth)
- **Anthropic** (API key or auth token)

Model note: while many providers/models are supported, for the best experience use the strongest latest-generation model available to you. See [Onboarding](#quick-start).

Models config + CLI: [Providers reference](docs/reference/api/providers-reference.md)
Auth profile rotation (OAuth vs API keys) + failover: [Model failover](docs/reference/api/providers-reference.md)

## Install (recommended)

Runtime: Rust stable toolchain. Single binary, no runtime dependencies.

### One-click bootstrap

```bash
git clone https://github.com/zeroclaw-labs/zeroclaw.git
cd zeroclaw
./install.sh
```

`metaclaw onboard` runs automatically after install to configure your workspace and provider.

## Quick start (TL;DR)

Full beginner guide (auth, pairing, channels): [Getting started](docs/setup-guides/one-click-bootstrap.md)

```bash
# Install + onboard
./install.sh --api-key "sk-..." --provider openrouter

# Start the gateway (webhook server + web dashboard)
metaclaw gateway                # default: 127.0.0.1:42617
metaclaw gateway --port 0       # random port (security hardened)

# Talk to the assistant
metaclaw agent -m "Hello, MetaClaw!"

# Interactive mode
metaclaw agent

# Start full autonomous runtime (gateway + channels + cron + hands)
metaclaw daemon

# Check status
metaclaw status

# Run diagnostics
metaclaw doctor
```

Upgrading? Run `metaclaw doctor` after updating.

### From source (development)

```bash
git clone https://github.com/zeroclaw-labs/zeroclaw.git
cd zeroclaw

cargo build --release --locked
cargo install --path . --force --locked

metaclaw onboard
```

> **Dev fallback (no global install):** prefix commands with `cargo run --release --` (example: `cargo run --release -- status`).

## Migrating from OpenClaw

MetaClaw puede importar tu workspace de OpenClaw, memoria y configuración:

```bash
# Preview what will be migrated (safe, read-only)
metaclaw migrate openclaw --dry-run

# Run the migration
metaclaw migrate openclaw
```

This migrates your memory entries, workspace files, and configuration from `~/.openclaw/` to `~/.metaclaw/`. Config is converted from JSON to TOML automatically.

## Security defaults (DM access)

MetaClaw se conecta a superficies de mensajería reales. Trata los DMs entrantes como input no confiable.

Full security guide: [SECURITY.md](SECURITY.md)

Default behavior on all channels:

- **DM pairing** (default): unknown senders receive a short pairing code and the bot does not process their message.
- Approve with: `metaclaw pairing approve <channel> <code>` (then the sender is added to a local allowlist).
- Public inbound DMs require an explicit opt-in in `config.toml`.
- Run `metaclaw doctor` to surface risky or misconfigured DM policies.

**Autonomy levels:**

| Level                  | Behavior                                                 |
| ---------------------- | -------------------------------------------------------- |
| `ReadOnly`             | Agent can observe but not act                            |
| `Supervised` (default) | Agent acts with approval for medium/high risk operations |
| `Full`                 | Agent acts autonomously within policy bounds             |

**Sandboxing layers:** workspace isolation, path traversal blocking, command allowlisting, forbidden paths (`/etc`, `/root`, `~/.ssh`), rate limiting (max actions/hour, cost/day caps).

<!-- BEGIN:WHATS_NEW -->
<!-- END:WHATS_NEW -->

### 📢 Announcements

> **Aviso**: Este es un fork personal de MetaClaw. Para anuncios oficiales de ZeroClaw, consulta [zeroclaw-labs/zeroclaw](https://github.com/zeroclaw-labs/zeroclaw).

| Date (UTC) | Level  | Notice                                                           |
| ---------- | ------ | ---------------------------------------------------------------- |
| 2026-04-02 | _Info_ | Este repositorio es un fork personal de ZeroClaw para uso local. |

## Highlights

- **Lean Runtime by Default** — common CLI and status workflows run in a few-megabyte memory envelope on release builds.
- **Cost-Efficient Deployment** — designed for $10 boards and small cloud instances, no heavyweight runtime dependencies.
- **Fast Cold Starts** — single-binary Rust runtime keeps command and daemon startup near-instant.
- **Portable Architecture** — one binary across ARM, x86, and RISC-V with swappable providers/channels/tools.
- **Local-first Gateway** — single control plane for sessions, channels, tools, cron, SOPs, and events.
- **Multi-channel inbox** — WhatsApp, Telegram, Slack, Discord, Signal, iMessage, Matrix, IRC, Email, Bluesky, Nostr, Mattermost, Nextcloud Talk, DingTalk, Lark, QQ, Reddit, LinkedIn, Twitter, MQTT, WeChat Work, WebSocket, and more.
- **Multi-agent orchestration (Hands)** — autonomous agent swarms that run on schedule and grow smarter over time.
- **Standard Operating Procedures (SOPs)** — event-driven workflow automation with MQTT, webhook, cron, and peripheral triggers.
- **Web Dashboard** — React 19 + Vite web UI with real-time chat, memory browser, config editor, cron manager, and tool inspector.
- **Hardware peripherals** — ESP32, STM32 Nucleo, Arduino, Raspberry Pi GPIO via the `Peripheral` trait.
- **First-class tools** — shell, file I/O, browser, git, web fetch/search, MCP, Jira, Notion, Google Workspace, and 70+ more.
- **Lifecycle hooks** — intercept and modify LLM calls, tool executions, and messages at every stage.
- **Skills platform** — bundled, community, and workspace skills with security auditing.
- **Tunnel support** — Cloudflare, Tailscale, ngrok, OpenVPN, and custom tunnels for remote access.

### Why pick MetaClaw

- **Lean by default:** small Rust binary, fast startup, low memory footprint.
- **Secure by design:** pairing, strict sandboxing, explicit allowlists, workspace scoping.
- **Fully swappable:** core systems are traits (providers, channels, tools, memory, tunnels).
- **No lock-in:** OpenAI-compatible provider support + pluggable custom endpoints.
- **Personalizado:** fork con modificaciones específicas para uso personal.

## Benchmark Snapshot (MetaClaw vs OpenClaw, Reproducible)

Local machine quick benchmark (macOS arm64, Feb 2026) normalized for 0.8GHz edge hardware.

|                           | OpenClaw      | NanoBot        | PicoClaw        | MetaClaw 🦀          |
| ------------------------- | ------------- | -------------- | --------------- | -------------------- |
| **Language**              | TypeScript    | Python         | Go              | **Rust**             |
| **RAM**                   | > 1GB         | > 100MB        | < 10MB          | **< 5MB**            |
| **Startup (0.8GHz core)** | > 500s        | > 30s          | < 1s            | **< 10ms**           |
| **Binary Size**           | ~28MB (dist)  | N/A (Scripts)  | ~8MB            | **~8.8 MB**          |
| **Cost**                  | Mac Mini $599 | Linux SBC ~$50 | Linux Board $10 | **Any hardware $10** |

> Notes: MetaClaw results are measured on release builds using `/usr/bin/time -l`. OpenClaw requires Node.js runtime (typically ~390MB additional memory overhead), while NanoBot requires Python runtime. PicoClaw and MetaClaw are static binaries. The RAM figures above are runtime memory; build-time compilation requirements are higher.

<p align="center">
  <img src="docs/assets/zeroclaw-comparison.jpeg" alt="MetaClaw vs OpenClaw Comparison" width="800" />
</p>

### Reproducible local measurement

```bash
cargo build --release
ls -lh target/release/metaclaw

/usr/bin/time -l target/release/metaclaw --help
/usr/bin/time -l target/release/metaclaw status
```

## Everything we built so far

### Core platform

- Gateway HTTP/WS/SSE control plane with sessions, presence, config, cron, webhooks, web dashboard, and pairing.
- CLI surface: `gateway`, `agent`, `onboard`, `doctor`, `status`, `service`, `migrate`, `auth`, `cron`, `channel`, `skills`.
- Agent orchestration loop with tool dispatch, prompt construction, message classification, and memory loading.
- Session model with security policy enforcement, autonomy levels, and approval gating.
- Resilient provider wrapper with failover, retry, and model routing across 20+ LLM backends.

### Channels

Channels: WhatsApp (native), Telegram, Slack, Discord, Signal, iMessage, Matrix, IRC, Email, Bluesky, DingTalk, Lark, Mattermost, Nextcloud Talk, Nostr, QQ, Reddit, LinkedIn, Twitter, MQTT, WeChat Work, WATI, Mochat, Linq, Notion, WebSocket, ClawdTalk.

Feature-gated: Matrix (`channel-matrix`), Lark (`channel-lark`), Nostr (`channel-nostr`).

### Web dashboard

React 19 + Vite 6 + Tailwind CSS 4 web dashboard served directly from the Gateway:

- **Dashboard** — system overview, health status, uptime, cost tracking
- **Agent Chat** — interactive chat with the agent
- **Memory** — browse and manage memory entries
- **Config** — view and edit configuration
- **Cron** — manage scheduled tasks
- **Tools** — browse available tools
- **Logs** — view agent activity logs
- **Cost** — token usage and cost tracking
- **Doctor** — system health diagnostics
- **Integrations** — integration status and setup
- **Pairing** — device pairing management

### Firmware targets

| Target       | Platform             | Purpose                      |
| ------------ | -------------------- | ---------------------------- |
| ESP32        | Espressif ESP32      | Wireless peripheral agent    |
| ESP32-UI     | ESP32 + Display      | Agent with visual interface  |
| STM32 Nucleo | STM32 (ARM Cortex-M) | Industrial peripheral        |
| Arduino      | Arduino              | Basic sensor/actuator bridge |
| Uno Q Bridge | Arduino Uno          | Serial bridge to agent       |

### Tools + automation

- **Core:** shell, file read/write/edit, git operations, glob search, content search
- **Web:** browser control, web fetch, web search, screenshot, image info, PDF read
- **Integrations:** Jira, Notion, Google Workspace, Microsoft 365, LinkedIn, Composio, Pushover, Weather (wttr.in)
- **MCP:** Model Context Protocol tool wrapper + deferred tool sets
- **Scheduling:** cron add/remove/update/run, schedule tool
- **Memory:** recall, store, forget, knowledge, project intel
- **Advanced:** delegate (agent-to-agent), swarm, model switch/routing, security ops, cloud ops
- **Hardware:** board info, memory map, memory read (feature-gated)

### Runtime + safety

- **Autonomy levels:** ReadOnly, Supervised (default), Full.
- **Sandboxing:** workspace isolation, path traversal blocking, command allowlists, forbidden paths, Landlock (Linux), Bubblewrap.
- **Rate limiting:** max actions per hour, max cost per day (configurable).
- **Approval gating:** interactive approval for medium/high risk operations.
- **E-stop:** emergency shutdown capability.
- **129+ security tests** in automated CI.

### Ops + packaging

- Web dashboard served directly from the Gateway.
- Tunnel support: Cloudflare, Tailscale, ngrok, OpenVPN, custom command.
- Docker runtime adapter for containerized execution.
- Pre-built binaries for Linux (x86_64, aarch64, armv7), macOS (x86_64, aarch64), Windows (x86_64).

## Configuration

Minimal `~/.metaclaw/config.toml`:

```toml
default_provider = "anthropic"
api_key = "sk-ant-..."
```

Full configuration reference: [docs/reference/api/config-reference.md](docs/reference/api/config-reference.md).

### Channel configuration

**Telegram:**

```toml
[channels.telegram]
bot_token = "123456:ABC-DEF..."
```

**Discord:**

```toml
[channels.discord]
token = "your-bot-token"
```

**Slack:**

```toml
[channels.slack]
bot_token = "xoxb-..."
app_token = "xapp-..."
```

**WhatsApp:**

```toml
[channels.whatsapp]
enabled = true
```

**Matrix:**

```toml
[channels.matrix]
homeserver_url = "https://matrix.org"
username = "@bot:matrix.org"
password = "..."
```

**Signal:**

```toml
[channels.signal]
phone_number = "+1234567890"
```

### Tunnel configuration

```toml
[tunnel]
kind = "cloudflare"  # or "tailscale", "ngrok", "openvpn", "custom", "none"
```

Details: [Channel reference](docs/reference/api/channels-reference.md) · [Config reference](docs/reference/api/config-reference.md)

### Runtime support (current)

- **`native`** (default) — direct process execution, fastest path, ideal for trusted environments.
- **`docker`** — full container isolation, enforced security policies, requires Docker.

Set `runtime.kind = "docker"` for strict sandboxing or network isolation.

## Subscription Auth (OpenAI Codex / Claude Code / Gemini)

MetaClaw supports subscription-native auth profiles (multi-account, encrypted at rest).

- Store file: `~/.metaclaw/auth-profiles.json`
- Encryption key: `~/.metaclaw/.secret_key`
- Profile id format: `<provider>:<profile_name>` (example: `openai-codex:work`)

```bash
# OpenAI Codex OAuth (ChatGPT subscription)
metaclaw auth login --provider openai-codex --device-code

# Gemini OAuth
metaclaw auth login --provider gemini --profile default

# Anthropic setup-token
metaclaw auth paste-token --provider anthropic --profile default --auth-kind authorization

# Check / refresh / switch profile
metaclaw auth status
metaclaw auth refresh --provider openai-codex --profile default
metaclaw auth use --provider openai-codex --profile work

# Run the agent with subscription auth
metaclaw agent --provider openai-codex -m "hello"
metaclaw agent --provider anthropic -m "hello"
```

## Agent workspace + skills

Workspace root: `~/.metaclaw/workspace/` (configurable via config).

Injected prompt files:

- `IDENTITY.md` — agent personality and role
- `USER.md` — user context and preferences
- `MEMORY.md` — long-term facts and lessons
- `AGENTS.md` — session conventions and initialization rules
- `SOUL.md` — core identity and operating principles

Skills: `~/.metaclaw/workspace/skills/<skill>/SKILL.md` or `SKILL.toml`.

```bash
# List installed skills
metaclaw skills list

# Install from git
metaclaw skills install https://github.com/user/my-skill.git

# Security audit before install
metaclaw skills audit https://github.com/user/my-skill.git

# Remove a skill
metaclaw skills remove my-skill
```

## CLI commands

```bash
# Workspace management
metaclaw onboard              # Guided setup wizard
metaclaw status               # Show daemon/agent status
metaclaw doctor               # Run system diagnostics

# Gateway + daemon
metaclaw gateway              # Start gateway server (127.0.0.1:42617)
metaclaw daemon               # Start full autonomous runtime

# Agent
metaclaw agent                # Interactive chat mode
metaclaw agent -m "message"   # Single message mode

# Service management
metaclaw service install      # Install as OS service (launchd/systemd)
metaclaw service start|stop|restart|status

# Channels
metaclaw channel list         # List configured channels
metaclaw channel doctor       # Check channel health
metaclaw channel bind-telegram 123456789

# Cron + scheduling
metaclaw cron list            # List scheduled jobs
metaclaw cron add "*/5 * * * *" --prompt "Check system health"
metaclaw cron remove <id>

# Memory
metaclaw memory list          # List memory entries
metaclaw memory get <key>     # Retrieve a memory
metaclaw memory stats         # Memory statistics

# Auth profiles
metaclaw auth login --provider <name>
metaclaw auth status
metaclaw auth use --provider <name> --profile <profile>

# Hardware peripherals
metaclaw hardware discover    # Scan for connected devices
metaclaw peripheral list      # List connected peripherals
metaclaw peripheral flash     # Flash firmware to device

# Migration
metaclaw migrate openclaw --dry-run
metaclaw migrate openclaw

# Shell completions
source <(metaclaw completions bash)
metaclaw completions zsh > ~/.zfunc/_metaclaw
```

Full commands reference: [docs/reference/cli/commands-reference.md](docs/reference/cli/commands-reference.md)

## Prerequisites

<details>
<summary><strong>Windows</strong></summary>

#### Required

1. **Visual Studio Build Tools** (provides the MSVC linker and Windows SDK):

   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```

   During installation (or via the Visual Studio Installer), select the **"Desktop development with C++"** workload.

2. **Rust toolchain:**

   ```powershell
   winget install Rustlang.Rustup
   ```

   After installation, open a new terminal and run `rustup default stable` to ensure the stable toolchain is active.

3. **Verify** both are working:
   ```powershell
   rustc --version
   cargo --version
   ```

#### Optional

- **Docker Desktop** — required only if using the [Docker sandboxed runtime](#runtime-support-current) (`runtime.kind = "docker"`). Install via `winget install Docker.DockerDesktop`.

</details>

<details>
<summary><strong>Linux / macOS</strong></summary>

#### Required

1. **Build essentials:**
   - **Linux (Debian/Ubuntu):** `sudo apt install build-essential pkg-config`
   - **Linux (Fedora/RHEL):** `sudo dnf group install development-tools && sudo dnf install pkg-config`
   - **macOS:** Install Xcode Command Line Tools: `xcode-select --install`

2. **Rust toolchain:**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   See [rustup.rs](https://rustup.rs) for details.

3. **Verify** both are working:
   ```bash
   rustc --version
   cargo --version
   ```

#### One-Line Installer

Or skip the steps above and install everything (system deps, Rust, MetaClaw) in a single command:

```bash
curl -LsSf https://raw.githubusercontent.com/zeroclaw-labs/zeroclaw/master/install.sh | bash
```

#### Compilation resource requirements

Building from source needs more resources than running the resulting binary:

| Resource       | Minimum | Recommended |
| -------------- | ------- | ----------- |
| **RAM + swap** | 2 GB    | 4 GB+       |
| **Free disk**  | 6 GB    | 10 GB+      |

If your host is below the minimum, use pre-built binaries:

```bash
./install.sh --prefer-prebuilt
```

To require binary-only install with no source fallback:

```bash
./install.sh --prebuilt-only
```

#### Optional

- **Docker** — required only if using the [Docker sandboxed runtime](#runtime-support-current) (`runtime.kind = "docker"`). Install via your package manager or [docker.com](https://docs.docker.com/engine/install/).

> **Note:** The default `cargo build --release` uses `codegen-units=1` to lower peak compile pressure. For faster builds on powerful machines, use `cargo build --profile release-fast`.

</details>

## Docs

Use these when you're past the onboarding flow and want the deeper reference.

- Start with the [docs index](docs/README.md) for navigation and "what's where."
- Read the [architecture overview](docs/architecture.md) for the full system model.
- Use the [configuration reference](docs/reference/api/config-reference.md) when you need every key and example.
- Run the Gateway by the book with the [operational runbook](docs/ops/operations-runbook.md).
- Follow [MetaClaw Onboard](#quick-start) for a guided setup.
- Debug common failures with the [troubleshooting guide](docs/ops/troubleshooting.md).
- Review [security guidance](docs/security/README.md) before exposing anything.

### Reference docs

- Documentation hub: [docs/README.md](docs/README.md)
- Unified docs TOC: [docs/SUMMARY.md](docs/SUMMARY.md)
- Commands reference: [docs/reference/cli/commands-reference.md](docs/reference/cli/commands-reference.md)
- Config reference: [docs/reference/api/config-reference.md](docs/reference/api/config-reference.md)
- Providers reference: [docs/reference/api/providers-reference.md](docs/reference/api/providers-reference.md)
- Channels reference: [docs/reference/api/channels-reference.md](docs/reference/api/channels-reference.md)
- Operations runbook: [docs/ops/operations-runbook.md](docs/ops/operations-runbook.md)
- Troubleshooting: [docs/ops/troubleshooting.md](docs/ops/troubleshooting.md)

### Collaboration docs

> **Nota**: Este es un fork personal. Para contribución al proyecto ZeroClaw principal, consulta [CONTRIBUTING.md en zeroclaw-labs/zeroclaw](https://github.com/zeroclaw-labs/zeroclaw/blob/master/CONTRIBUTING.md).

- Contribution guide: [CONTRIBUTING.md](CONTRIBUTING.md)
- Security disclosure policy: [SECURITY.md](SECURITY.md)

### Deployment + operations

- Network deployment guide: [docs/ops/network-deployment.md](docs/ops/network-deployment.md)
- Proxy agent playbook: [docs/ops/proxy-agent-playbook.md](docs/ops/proxy-agent-playbook.md)
- Hardware guides: [docs/hardware/README.md](docs/hardware/README.md)

## Fork de ZeroClaw

MetaClaw es un fork personal del proyecto ZeroClaw mantenido para uso local con modificaciones específicas.

- **Repositorio upstream**: [zeroclaw-labs/zeroclaw](https://github.com/zeroclaw-labs/zeroclaw)
- **Propósito**: Uso personal/interno
- **Compatibilidad**: Mantiene compatibilidad con API y configuración de ZeroClaw

---

## License

MetaClaw es un fork de ZeroClaw y hereda las mismas licencias:

ZeroClaw is dual-licensed for maximum openness and contributor protection:

| License                      | Use case                                                |
| ---------------------------- | ------------------------------------------------------- |
| [MIT](LICENSE-MIT)           | Open-source, research, academic, personal use           |
| [Apache 2.0](LICENSE-APACHE) | Patent protection, institutional, commercial deployment |

### Trademark

The **ZeroClaw** name and logo are trademarks of ZeroClaw Labs. This fork uses the name **MetaClaw** for personal use. See [TRADEMARK.md](docs/maintainers/trademark.md) for ZeroClaw trademark policy.

---

**MetaClaw** — Fork personal de ZeroClaw. Zero overhead. Zero compromise. 🦀
