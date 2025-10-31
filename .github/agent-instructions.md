# Omniscient Shell – Agent Instructions Spec (v0.1.1)

## 🎯 Purpose
Define how agents are described, executed, sandboxed, and interact with the Omniscient Shell. This spec ensures:
- **Consistency:** All agents follow the same lifecycle and manifest schema.
- **Security:** Agents run with least privilege, explicit consent, and isolation.
- **Extensibility:** New agents can be added without modifying the core shell.
- **Futureproofing:** Versioned manifests and event streams allow evolution without breaking compatibility.

---

## 1. Agent Manifest (Schema v0.1)

Each agent must include a `manifest.toml` file:

```toml
schema_version = "0.1"
name = "Claude Code"
version = "0.1.0"
entry = "claude_agent.wasm"
sandbox = "wasm"              # wasm | native

capabilities = ["codegen", "files.read", "files.write", "network", "media.preview"]
oauth_scopes = ["github:repo", "google:drive.readonly"]

resources.cpu = "500m"
resources.mem = "512Mi"

ui.hints = ["streaming", "diff", "preview"]
```

### Required fields
- **schema_version:** Version of manifest schema.
- **name/version:** Unique identifier.
- **entry:** Executable or WASM module.
- **sandbox:** Execution mode (`wasm` preferred, `native` allowed with explicit user consent).
- **capabilities:** Declared permissions (see section 2).
- **oauth_scopes:** External API scopes requested.
- **resources:** CPU/memory quotas.
- **ui.hints:** UI integration hints (streaming, preview, diff, etc.).

---

## 2. Capabilities Model

Agents must declare **capabilities** they require. The shell enforces **default deny**.

### Core capability categories
- **files.read / files.write:** Access to workspace files.
- **network:** Outbound HTTP(S) requests.
- **media.preview:** Request rendering of images/video.
- **codegen:** Generate code artifacts.
- **memory:** Persist agent-specific state.
- **oauth:provider.scope:** Request OAuth tokens via broker.

### Consent model
- **Per-capability sessions:** Time-bounded grants, revocable at any time.
- **Sensitive actions:** Per-action prompts for high-risk operations (e.g., network writes, external process execution).
- **Guardrails:** Revocation must not silently break functionality; fallback paths required.

---

## 3. Agent Lifecycle

### States
1. **Registered:** Manifest discovered and validated.
2. **Initialized:** Agent runtime started (WASM instance or native subprocess).
3. **Active:** Agent can receive inputs and stream outputs.
4. **Suspended:** Paused due to resource limits or user action.
5. **Terminated:** Gracefully shut down or force-killed.

### Lifecycle hooks
- **start(ctx):** Initialize agent with context (workspace, config, granted capabilities).
- **invoke(input):** Handle user/system input; return streaming output.
- **shutdown():** Cleanup resources, flush state.

---

## 4. Event Protocol (v0.1)

Agents communicate with the shell via structured events.

### Event types
- **input:** `{ agent_id, prompt, context_refs[] }`
- **output:** `{ agent_id, chunk_id, content_type, data }`
- **artifact:** `{ id, kind, path, preview_hint }`
- **consent_request:** `{ capability, reason, duration_s }`
- **consent_grant/revoke:** `{ capability, expires_at }`
- **error:** `{ code, message, hint }`
- **state_update:** `{ key, value, scope }`

### Streaming
- Outputs are chunked; shell assembles into cards.
- Backpressure enforced via token-bucket throttling.

---

## 5. Sandboxing & Execution

### WASM-first (preferred)
- **Runtime:** WASI-compliant host.
- **Syscalls:** Capability-gated; deny-by-default.
- **Isolation:** Memory and CPU quotas enforced.

### Native (opt-in)
- **Isolation:** OS-level (Job Objects on Windows, cgroups on Linux, sandbox-exec on macOS).
- **Quotas:** CPU/mem/timeouts enforced.
- **Filesystem:** Scoped to workspace root.
- **Network:** Domain allowlist; explicit consent required.

---

## 6. OAuth Integration

- **Broker:** Mediates OAuth flows (device/PKCE).
- **Vault:** Stores tokens encrypted (OS keychain preferred).
- **Agent access:** Agents never see raw tokens; they receive scoped handles.
- **Revocation:** Immediate; logged in consent ledger.

---

## 7. Persistence & State

- **Per-agent KV store:** For memory and preferences.
- **Shared session scratchpad:** For inter-agent collaboration.
- **Event log:** Append-only; reconstructable state.
- **Artifacts:** Auto-saved to workspace per retention policy.

---

## 8. UI Integration

- **Cards:** Agents stream into cards (code, diff, preview, log).
- **Hints:** Agents can suggest UI modes (e.g., preview image, diff view).
- **Consent cards:** Inline prompts for capability grants.
- **Ledger view:** Shows agent actions, grants, revocations.

---

## 9. Security & Auditing

- **Zero trust:** No implicit privileges.
- **Explicit consent:** All capabilities gated.
- **Audit log:** Append-only ledger of agent actions, grants, revocations.
- **Exportable:** Logs can be exported for review; secrets redacted.

---

## 10. Versioning & Compatibility

- **Schema versions:** Manifests, configs, and events include `schema_version`.
- **Backward compatibility:** Shell must reject unsupported versions with clear error.
- **Migration tools:** Provide utilities to upgrade manifests/configs.

---

## ✅ Acceptance Criteria for Agents

- Must include valid `manifest.toml` with schema_version.
- Must run in WASM sandbox unless explicitly allowed native.
- Must declare all required capabilities; no hidden privileges.
- Must handle consent revocation gracefully.
- Must stream outputs in event protocol v0.1.
- Must persist state only via provided KV store.
- Must log all actions to event ledger.

---

## Implementation Notes

This specification is designed to be:
- **Language-agnostic:** Agents can be implemented in any language that compiles to WASM or runs natively.
- **Runtime-agnostic:** Can be implemented in PowerShell, Rust, Go, or any runtime with WASM support.
- **Transport-agnostic:** Events can be serialized as JSON, MessagePack, or Protocol Buffers.

### Reference Implementations
- **PowerShell Agent Host:** Native PowerShell implementation with WASI support
- **Rust Agent Runtime:** High-performance WASM host with capability enforcement
- **TypeScript Agent SDK:** Developer-friendly SDK for building agents

---

## Version History

- **v0.1.1** (2025-10-30): Initial specification
  - Core manifest schema
  - Capabilities model
  - Event protocol v0.1
  - Sandboxing requirements
  - OAuth integration model
