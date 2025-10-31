# Omniscient Shell

A cross-platform, AI-native companion shell extending PowerShell with visual fidelity, secure agent orchestration, and persistent workspaces.

## Features

### Phase 1: Core + TUI (✓ Implemented)
- ✅ Rust project structure with feature flags
- ✅ PowerShell integration layer
- ✅ TUI dashboard with panes (shell, agent, preview, log)
- ✅ Graphics backend negotiation (Notcurses → Kitty → Overlay)
- ✅ Config loader with schema v0.1 validation
- ✅ Theme system (NeoCyan default)

### Phase 2: Agents + Sandbox (✓ Implemented)
- ✅ WASM runtime host (WASI-compliant)
- ✅ Agent registry with manifest validation
- ✅ Capability model with default-deny enforcement
- ✅ Event protocol v0.1
- ✅ Native subprocess runner with OS-level isolation
- ✅ Streaming UI for agents

### Phase 3: OAuth + Vault (✓ Implemented)
- ✅ OAuth broker (device code + PKCE flows)
- ✅ Provider adapters (GitHub, Google)
- ✅ Token vault with OS keychain integration
- ✅ Scoped handle system
- ✅ Consent ledger
- ✅ Consent UI cards

### Phase 4: Media + Workspaces (✓ Implemented)
- ✅ FFmpeg integration (structure ready)
- ✅ Media cache with intelligent pruning
- ✅ Inline image/video rendering
- ✅ Workspace selection and artifact resolution
- ✅ Retention policies
- ✅ SQLite artifact index

### Phase 5: Policies + QA (✓ Implemented)
- ✅ Notification system with profiles
- ✅ Policy engine
- ✅ Comprehensive test suites
- ✅ CI matrix (Windows/macOS/Linux)
- ✅ Structured error handling with recovery hints
- ✅ Telemetry (opt-in only, no secrets)

## Advanced Features (New)
- ✅ **CI/CD Pipeline** - GitHub Actions workflow for all platforms
- ✅ **Structured Error Recovery** - Automatic recovery actions and fallbacks
- ✅ **Telemetry System** - Opt-in performance metrics (no secrets)
- ✅ **Command Palette** - Interactive command discovery and execution
- ✅ **Schema Migration Tools** - Database versioning with rollback support
- ✅ **Security Audit** - Automated dependency scanning in CI

## Building

### Prerequisites
- Rust 1.75+ with cargo
- PowerShell 7+ (pwsh)

### Build
```bash
cargo build --release
```

### Feature Flags
```bash
# Build with Kitty backend only (minimal)
cargo build --no-default-features --features kitty

# Build with WASM support
cargo build --features wasm

# Build with all features (requires system dependencies)
cargo build --all-features
```

## Running

```bash
# Start the dashboard
./target/release/omniscient-shell

# Or use the short alias
./target/release/omni
```

### Keyboard Shortcuts
- `q` or `Esc` - Quit
- `Ctrl+C` - Force quit

## Configuration

Configuration is stored at `~/.omniscient/config.toml` (created automatically on first run).

### Example Configuration (Schema v0.1)
```toml
version = "0.1"

[workspace]
detection = "explicit"
auto_save = true

[graphics]
preferred = "kitty"
fallback = ["overlay"]
auto_benchmark = true

[theme]
name = "NeoCyan"
background = "#0b0e10"
foreground = "#c9d1d9"
accent = "#00d1ff"

[agents]
enabled = []
sandbox_default = "wasm"
policy = "user-choice"

[retention]
always_persist = ["diff", "log"]
ephemeral = ["preview", "scratch"]
days = 30
max_mb = 1024
```

## Architecture

```
src/
├── shell/          # PowerShell integration, command router
├── tui/            # Layout engine, panes, cards, themes
├── graphics/       # Notcurses/Kitty/Overlay backends
├── media/          # FFmpeg pipeline, cache (Phase 4)
├── agents/         # WASM host, native runner, registry (Phase 2)
├── oauth/          # Broker, providers, vault (Phase 3)
├── state/          # SQLite, event ledger, KV store
├── security/       # Capabilities, consent, isolation (Phase 2)
├── workspace/      # Selection, artifacts, retention (Phase 4)
├── notifications/  # Notifier abstraction, profiles (Phase 5)
├── platform/       # Cross-platform abstractions
└── utils/          # Config, logging, errors
```

## Development

### Running Tests
```bash
cargo test
```

### Linting
```bash
cargo clippy
```

### Formatting
```bash
cargo fmt
```

## Security

- **Capability-based security**: Default deny, explicit grants
- **Sandboxing**: WASM-first, native with OS-level isolation
- **No secret leaks**: Tokens stored in OS keychain, agents receive handles only
- **Audit logging**: Append-only ledger of all actions

## Contributing

See [CONTRIBUTING.md](../.github/CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE.txt](../LICENSE.txt)
