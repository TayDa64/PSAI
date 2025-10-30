# Omniscient Shell Implementation Task

## Overview

Implement the Omniscient Shell - a cross-platform, AI-native companion shell that extends PowerShell with visual fidelity, secure agent orchestration, and persistent workspaces.

## Reference Documentation

This task requires following three specification documents:

1. **`.github/copilot-instructions.md`** - PowerShell integration patterns, build system, and development conventions
2. **`.github/agent-instructions.md`** - Agent manifest schema, capabilities model, lifecycle, and event protocol
3. **`.github/BUILD-INSTRUCTIONS.md`** - Complete architecture, interfaces, configuration schema, and acceptance criteria

## Primary Objectives

Build a production-ready shell that:

- Renders rich TUI with Notcurses (preferred) and Kitty graphics (fallback)
- Executes AI agents in WASM sandboxes with capability-based security
- Integrates OAuth for GitHub/Google with encrypted vault storage
- Provides explicit workspace management with retention policies
- Maintains event-sourced audit logs for all agent actions

## Implementation Approach

### Phase 1: Core + TUI (Week 1)
- Set up Rust project structure per `src/` layout in BUILD-INSTRUCTIONS
- Implement PowerShell integration layer (command router, process supervision)
- Build TUI layout engine with dashboard, panes, cards, sections
- Implement graphics backend negotiation (Notcurses → Kitty → Overlay fallback)
- Add config loader with validation and hot-reload
- **Acceptance:** Dashboard renders with all panes; graphics auto-select; config hot-reloads

### Phase 2: Agents + Sandbox (Week 2)
- Implement WASM runtime host (WASI-compliant)
- Build agent registry with manifest validation (schema v0.1)
- Create capability model with default-deny enforcement
- Implement event protocol (input, output, artifact, consent_request, error)
- Add native subprocess runner with OS-level isolation
- Build streaming UI for agent outputs into cards
- **Acceptance:** WASM agent streams to cards; capabilities gated; native agent under quotas

### Phase 3: OAuth + Vault (Week 3)
- Implement OAuth broker (device code and PKCE flows)
- Add provider adapters for GitHub and Google
- Build token vault with OS keychain integration (fallback: encrypted SQLite)
- Implement scoped handle system (agents never see raw tokens)
- Add consent ledger with grant/revoke tracking
- Build consent UI cards with inline prompts
- **Acceptance:** GitHub/Google flows complete; tokens vaulted; consent ledger functional

### Phase 4: Media + Workspaces (Week 4)
- Integrate FFmpeg for thumbnails, scaling, frame extraction
- Implement media cache with intelligent pruning
- Build inline image/video rendering with throttling
- Add workspace selection and artifact path resolution
- Implement retention policies (always-persist vs ephemeral)
- Build SQLite-backed artifact index
- **Acceptance:** Inline media renders at high resolution; workspace auto-saves per policy

### Phase 5: Policies + QA (Week 5)
- Implement notification system with profiles (minimal default)
- Add policy engine for retention, notifications, consent
- Write comprehensive test suites (unit, integration, security)
- Set up CI matrix (Windows/macOS/Linux)
- Implement structured error handling with recovery hints
- Add telemetry (opt-in, performance-only)
- **Acceptance:** All tests pass; CI green; policies functional; no security gaps

## Technical Constraints

- **Rust:** 1.75+ with cargo
- **Graphics:** Notcurses 3 (preferred), Kitty protocol (fallback), Ncurses (overlay)
- **Sandboxing:** WASM via WASI; native via Job Objects/cgroups/sandbox-exec
- **State:** SQLite with event-sourced ledger
- **PowerShell:** Integration via `build.psm1` patterns from copilot-instructions
- **Security:** Zero-trust, explicit consent, append-only audit logs

## Development Guidelines

### From copilot-instructions.md:
- Follow PowerShell build patterns (`Import-Module ./build.psm1`, `Start-PSBuild`)
- Use platform detection variables (`$IsWindows`, `$IsLinux`, `$IsMacOS`)
- Ensure cross-platform compatibility (Windows/Linux/macOS)

### From agent-instructions.md:
- All agents must have valid `manifest.toml` with schema_version 0.1
- Implement event protocol v0.1 for agent communication
- Enforce capability-based security (default deny)
- Handle consent revocation gracefully

### From BUILD-INSTRUCTIONS.md:
- Implement all interfaces per specifications (graphics backend, agent runtime, OAuth broker, vault, workspace)
- Follow config schema v0.1 exactly
- Implement graceful fallbacks (graphics, errors, revocations)
- Version all schemas and reject mismatches with clear errors

## Success Criteria

- [ ] All 5 phases meet acceptance criteria
- [ ] Cross-platform builds succeed (Windows/Linux/macOS)
- [ ] Graphics negotiate and render correctly on all backends
- [ ] WASM agents execute safely with capability enforcement
- [ ] OAuth flows complete without exposing secrets
- [ ] Workspace persistence follows retention policies
- [ ] Security tests validate sandbox isolation
- [ ] CI matrix passes all tests
- [ ] Documentation matches implementation

## Additional Notes

- **Feature flags:** Enable graphics backends (notcurses, kitty, overlay) and sandbox modes (wasm, native) via Cargo features
- **Error handling:** All errors must include recovery hints and surface in log pane
- **Versioning:** Reject schema version mismatches with migration guidance
- **Telemetry:** Disabled by default; opt-in only; never log secrets

## Questions to Resolve Before Starting

1. Should we create a new repository or implement within this PowerShell fork?
2. What is the preferred naming convention for the Rust crates?
3. Are there existing PowerShell integration libraries we should leverage?
4. Should the shell binary be named `omniscient-shell`, `omni`, or `osh`?
5. What is the target for first release (MVP vs full feature set)?

---

**Ready to proceed?** Please review the three reference documents and confirm the implementation approach before beginning Phase 1.
