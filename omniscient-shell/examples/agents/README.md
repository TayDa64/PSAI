# Example Agents

This directory contains example agent implementations demonstrating the Omniscient Shell agent system.

## Example Agent

A simple example agent showing the manifest format and structure.

### Manifest Fields

- `schema_version`: Must be "0.1"
- `name`: Human-readable agent name
- `version`: Semantic version
- `entry`: Entry point (WASM or executable)
- `sandbox`: "wasm" or "native"
- `capabilities`: List of required capabilities
- `oauth_scopes`: OAuth scopes needed
- `resources`: CPU and memory limits
- `ui.hints`: UI rendering hints

### Testing

You can test manifest parsing:

```bash
cargo test manifest
```

### Creating Your Own Agent

1. Create a directory under `examples/agents/`
2. Add a `manifest.toml` following the schema
3. Implement your agent in WASM or native code
4. Register with the shell's agent registry
