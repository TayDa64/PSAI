# Build instructions for the omniscient shell (v0.1.1)

A cross-platform, AI-native companion shell that extends PowerShell. This guide is versioned and modular, optimized for visual fidelity using Notcurses (primary) and Kitty graphics protocol (primary fallback), with secure agent orchestration, OAuth, and persistent workspaces.

---

## Scope and design choices

- **Purpose:** A visually expressive, agentic commandline that feels like a website—panes, cards, inline media—while retaining keyboard-first speed.
- **Graphics target:** Notcurses (preferred), Kitty (primary fallback), Ncurses overlay (secondary fallback). Sixel is legacy-only and disabled by default.
- **Sandboxing:** WASM-first for safety (WASI), with an opt-in native subprocess path for performance-critical agents under strict OS isolation.
- **Consent model:** Per-capability sessions with time bounds and revocation; sensitive actions may request per-action consent.
- **Persistence:** Config-driven, event-sourced, SQLite-backed state; explicit workspace selection; artifacts auto-save with retention policies.
- **Versioning:** All configs, manifests, and event streams include schema versions for compatibility.

---

## Prerequisites and environment

- **Languages & tools:**
  - **Rust:** 1.75+ with cargo
  - **Git:** Latest stable
  - **SQLite:** For state store
  - **pkg-config:** For library detection and linking

- **Graphics libraries:**
  - **Notcurses 3:** Required for the preferred renderer
  - **Kitty graphics protocol:** Supported in compatible terminals; no external library required
  - **Ncurses:** For overlay fallback and basic TUI primitives

- **Media pipeline:**
  - **FFmpeg:** Thumbnails, scaling, waveform/spectrogram, frame extraction, caching

- **Optional OS integrations:**
  - **Windows:** Win32 console APIs; optional system notifications
  - **macOS/Linux:** Standard terminal capability detection, notify-send/AppleScript optional

---

## Project structure

- **src/shell:** PowerShell integration, command router, process supervision, history, completions
- **src/tui:** Layout engine, panes, sections, cards, theme system, input handling
- **src/graphics:** Graphics backends (Notcurses, Kitty), capability negotiation, overlay fallback
- **src/media:** FFmpeg transforms, cache management, preview adapters
- **src/agents:** Agent runtime (WASM host, native subprocess runner), IPC/streaming, registry
- **src/oauth:** Broker (device/PKCE flows), provider adapters (GitHub/Google), token vault
- **src/state:** SQLite store, migrations, event-sourced ledger, per-agent KV, retention policies
- **src/security:** Capability model, consent UX, isolation controls, resource limits
- **src/workspace:** Explicit selection, artifact paths, persistence rules, policy integration
- **src/notifications:** Notifier abstraction, profiles, minimal default
- **src/platform:** Cross-platform process, filesystem, sandbox, and notification abstractions
- **src/utils:** Config loader, validation, logging, tracing, error model

---

## Configuration schema (v0.1)

Place at `~/.omniscient/config.toml`. All fields are validated at startup; invalid entries fall back to safe defaults.

```toml
version = "0.1"

[workspace]
detection = "explicit"        # explicit selection required for builds
root = "~/Projects/current"   # default workspace root (optional)
auto_save = true

[graphics]
preferred = "notcurses"       # options: notcurses, kitty
fallback = ["kitty", "overlay"]
auto_benchmark = true         # benchmark resolution/latency to confirm choice
legacy_support = []           # enable ["sixel"] only if explicitly requested

[layout.default]
preset = "dashboard"
panes = ["shell", "agent", "preview", "log"]

[theme]
name = "NeoCyan"
background = "#0b0e10"
foreground = "#c9d1d9"
accent = "#00d1ff"

[agents]
enabled = ["copilot", "gemini", "claude"]
sandbox_default = "wasm"      # wasm or native
native_allowed = ["local-llm", "gpu-heavy"]
policy = "user-choice"        # user selects preferred AI/provider

[retention]
always_persist = ["diff", "log"]
ephemeral = ["preview", "scratch"]
days = 30
max_mb = 1024

[oauth.providers.github]
client_id = "env:GITHUB_CLIENT_ID"
scopes = ["repo", "read:user"]
flow = "device_code"

[oauth.providers.google]
client_id = "env:GOOGLE_CLIENT_ID"
scopes = ["drive.readonly", "userinfo.email"]
flow = "device_code"

[vault]
backend = "os_keychain"       # fallback: "encrypted_sqlite"
auto_lock_minutes = 10
key_derivation = "argon2id"

[notifications]
profile = "minimal"
channels = ["tui", "system"]
```

---

## Interfaces and contracts

### Graphics backend interface

- **Trait methods:**
  - **init():** Initialize backend and negotiate capabilities
  - **capabilities():** Report supported resolution, color depth, animations, transparency
  - **render_image(region, img):** Inline image rendering with scaling
  - **render_video_frame(region, frame):** Frame rendering with throttling and backpressure
  - **clear_region(region):** Clear area for redraw
  - **supports_resolution(width, height):** Choose backend for highest effective resolution

- **Backends:**
  - **Notcurses:** Preferred; high-fidelity rendering and advanced widgets
  - **Kitty:** Primary fallback; robust inline graphics support
  - **Overlay:** Ncurses-based fallback for non-graphics terminals

- **Negotiation:**
  - **Rule:** Pick backend with highest verified resolution and lowest measured latency, respecting `graphics.preferred`.
  - **Benchmark:** Quick, non-intrusive probe at startup; cache results.

### Agent runtime

- **Transport:**
  - **WASM:** JSON-RPC over stdio with streaming chunks
  - **Native:** gRPC/domain sockets with streaming; isolated subprocess

- **Manifest (v0.1):**
  - **schema_version:** "0.1"
  - **name, version, entry, sandbox:** wasm or native
  - **capabilities:** e.g., files.read, network, media.preview
  - **oauth_scopes:** e.g., github:repo, google:drive.readonly
  - **resources:** cpu, mem quotas
  - **ui.hints:** streaming, diff, preview

- **Consent and capabilities:**
  - **Default deny:** Agents request capabilities; grants are time-bounded and revocable
  - **Sensitive actions:** Per-action prompt for network writes, external process exec, filesystem mutations outside workspace

### OAuth broker and vault

- **Providers:** GitHub and Google device/PKCE flows
- **Broker contract:**
  - **request_token(provider, scopes):** Initiate device/PKCE; return scoped token handle
  - **refresh(handle):** Rotate tokens securely
  - **revoke(handle):** Revoke and audit

- **Vault contract:**
  - **store(label, secret):** Encrypted at rest; no plaintext leaks to agents
  - **fetch(label):** Controlled by broker; agents receive handles, not secrets
  - **lock/unlock:** Auto-lock on idle; manual unlock with passphrase
  - **rotate():** Periodic key rotation per config

### Workspace and retention

- **Workspace provider:**
  - **select(root):** Explicit selection required for builds
  - **resolve_artifact_path(kind):** Deterministic paths per artifact type
  - **policy():** Apply retention rules (always persist diffs/logs; ephemeral for previews)

- **Retention policy:**
  - **should_persist(kind):** Enforce always-persist list
  - **ttl(kind):** Ephemeral lifetimes (days)
  - **prune(strategy):** Size/time-based pruning, bookmarks exempt

---

## Build and run

### Setup

- **Clone repository:**
  - git clone repository
  - cd repository

- **Install dependencies:**
  - **Notcurses:** Install platform packages or build from source
  - **Ncurses:** Install platform packages
  - **FFmpeg:** Install platform packages
  - **SQLite:** Install platform packages

- **Build:**
  - cargo build --release

- **Run:**
  - ./target/release/omniscient-shell

### Useful commands

- **Select workspace:**
  - omniscient-shell --workspace ~/Projects/current

- **Switch graphics backend:**
  - omniscient-shell --graphics notcurses
  - omniscient-shell --graphics kitty

- **Benchmark graphics:**
  - omniscient-shell --graphics-benchmark

- **Launch dashboard layout:**
  - omniscient-shell --layout dashboard

- **Authenticate provider:**
  - omniscient-shell --auth github

---

## Visual system: terminal-as-website

- **Layout primitives:**
  - **Panes:** Shell, agent console, preview, log
  - **Sections:** Status (OAuth, capabilities), actions, artifacts
  - **Cards:** Results, diffs, metrics, prompts; theme-aware borders and spacing
  - **Overlay:** Full-screen consent and media preview when inline rendering is unavailable

- **Rendering pipeline:**
  - **Feature detection:** Probe Notcurses/Kitty capabilities on startup
  - **Inline media:** Prefer high-resolution backend; throttle frame rate; cache thumbnails
  - **Fallbacks:** ASCII thumbnails and overlay previews for unsupported terminals
  - **Accessibility:** Configurable color contrast and minimal mode

- **Interaction patterns:**
  - **Keyboard-first:** Hotkeys for focus, actions, previews, consent, bookmarks
  - **Context menus:** Inline actions (open, copy, run, save)
  - **Streaming:** Agents stream into cards; partial results are diffable and actionable
  - **Bookmark & restore:** Session restore into dashboard; layout presets selectable

---

## Security and consent

- **Capability grants:**
  - **Per-capability sessions:** Time-bounded, revocable; grants cannot silently remove existing functionality
  - **Sensitive actions:** Per-action prompts for high-risk operations
  - **Guardrails:** Clear fallbacks when capabilities are revoked

- **Auditing:**
  - **Event ledger:** Append-only; filterable; exportable with secrets redacted
  - **User visibility:** Consent cards show scope details, duration, revoke action

- **Isolation:**
  - **WASM host:** Capability-gated syscalls; deny-by-default
  - **Native subprocess:** Job Objects (Windows), cgroups (Linux), sandbox-exec (macOS); CPU/mem/network quotas; filesystem scoping to workspace

---

## Persistence, artifacts, and retention

- **Auto-save:**
  - **Artifacts:** Code diffs and logs always persist to workspace; previews/scratch are ephemeral by policy
  - **Index:** SQLite-backed artifact index with metadata for previews and diffs

- **Retention:**
  - **Policies:** Enforced by type; background pruning with size/time thresholds
  - **Bookmarks:** Exempt from pruning
  - **Export:** Batch export to external folders or archives

---

## Developer workflow

- **Scaffold an agent:**
  - omniscient-shell --scaffold-agent <name>

- **Register an agent:**
  - Add manifest with schema_version, capabilities, oauth_scopes, sandbox mode
  - Implement handler using runtime-specific SDK
  - Test streaming outputs and capability requests

- **Extend OAuth:**
  - Add provider config, verify device/PKCE flows
  - Ensure broker issues scoped handles; refresh and revoke are audited

- **Build a widget:**
  - Implement theme-aware component; expose hotkeys/actions; write rendering tests
  - Validate performance with graphics benchmark

- **Media pipeline:**
  - Implement FFmpeg transformations; cache thumbnails and frames
  - Use backend capability checks for resolution selection

---

## Testing and QA

- **Unit:**
  - **Config validation:** Version checks, defaults, fallback behavior
  - **Graphics negotiation:** Preferred selection, latency/resolution benchmarking
  - **Vault operations:** Lock/unlock, rotate, store/fetch handles
  - **Capability checks:** Default deny, per-capability sessions

- **Integration:**
  - **WASM streaming:** Backpressure, partial results, UI updates
  - **Native isolation:** Resource limits, filesystem scoping, network egress control
  - **OAuth device flow:** Non-blocking UX, refresh/revoke, audit entries
  - **Workspace retention:** Auto-save, pruning, bookmark immunity

- **Security:**
  - **Sandbox escapes:** Deny unexpected syscalls
  - **Token hygiene:** No plaintext secrets in agent processes
  - **Consent revocation:** Immediate effect, graceful agent degradation

- **Cross-platform CI:**
  - Windows/macOS/Linux matrices; terminal capability detection; graphics fallbacks; performance gates

---

## Acceptance criteria by phase

- **Phase 1 (Core + TUI):**
  - **Dashboard renders:** Shell, agent console stub, preview, log
  - **Graphics negotiation:** Notcurses selected if available; Kitty fallback; overlay if none
  - **Config hot-reload:** Themes and layouts update without crash

- **Phase 2 (Agents + Sandbox):**
  - **WASM agent:** Streams into cards; capability requests visible and gated
  - **Native agent:** Runs under OS isolation; resource quotas enforced

- **Phase 3 (OAuth + Vault):**
  - **GitHub/Google:** Device/PKCE flows complete; tokens vaulted and provisioned via handles
  - **Consent ledger:** Grants/revocations logged; UI cards functional

- **Phase 4 (Media + Workspaces):**
  - **Inline images/video:** High-resolution with frame-rate throttle and caching
  - **Workspace auto-save:** Diffs/logs persist; previews ephemeral per policy

- **Phase 5 (Policies + QA):**
  - **Policy-driven retention/notifications:** Profiles selectable; minimal default
  - **CI matrix:** Cross-platform tests pass; security checks validated

---

## Notes for the coding agent

- **Feature flags:**
  - **Graphics:** notcurses, kitty, overlay
  - **Sandbox:** wasm, native

- **Error handling:**
  - **Structured errors:** Include recovery hints; surface in log pane and minimal notifications
  - **Graceful fallback:** If preferred backend unsupported, auto-select next and continue

- **Telemetry:**
  - **Disabled by default:** Optional performance-only metrics; no secrets; user consent required

- **Versioning:**
  - **Reject mismatches:** Clear errors if `version` or `schema_version` differs; offer migration guidance

---
