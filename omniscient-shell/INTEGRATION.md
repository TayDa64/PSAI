# Omniscient Shell - PowerShell Integration Guide

## Overview

The Omniscient Shell extends PowerShell without replacing it. It provides a visual, AI-native interface while preserving all existing PowerShell functionality.

## Integration Architecture

### Command Router

Commands entered in the Omniscient Shell are routed to the appropriate handler:

```
User Input
    ↓
Command Router
    ↓
    ├─→ PowerShell (standard commands)
    ├─→ Omniscient Shell (omni: prefix)
    └─→ AI Agents (configured agents)
```

### PowerShell Integration Layer

The shell integration module (`src/shell/integration.rs`) provides:

1. **Command Execution**: Execute PowerShell commands via `pwsh` process
2. **Process Supervision**: Monitor and manage PowerShell instances
3. **History Management**: Track command history across sessions
4. **Completions**: PowerShell tab completion integration (Phase 2)

### Preserving Existing Functionality

Following the principles from `.github/copilot-instructions.md`, the Omniscient Shell:

- **Does NOT modify** existing PowerShell binaries or installations
- **Does NOT interfere** with PowerShell modules or commands
- **Runs as a separate process** that spawns PowerShell instances
- **Uses standard PowerShell** flags like `-NoProfile` and `-NonInteractive`
- **Respects PowerShell** execution policies and security settings

## Usage Examples

### Running PowerShell Commands

All standard PowerShell commands work as expected:

```powershell
# List files
Get-ChildItem

# Run PowerShell scripts
.\build.ps1

# Import modules
Import-Module ./build.psm1
Start-PSBuild
```

### Omniscient Shell Commands

Commands with the `omni:` prefix are handled by the shell:

```bash
# Workspace management (Phase 4)
omni:workspace select ~/Projects/myproject

# Agent invocation (Phase 2)
omni:agent invoke copilot "explain this code"

# Configuration
omni:config reload
```

## Configuration

The shell reads configuration from `~/.omniscient/config.toml` but does not modify any PowerShell configuration files.

## PowerShell Detection

The shell automatically detects PowerShell using the following priority:

1. `pwsh` (PowerShell 7+) - Preferred
2. `powershell` (Windows PowerShell 5.1) - Fallback on Windows

## Build Integration

When building PowerShell from source (as documented in `.github/copilot-instructions.md`), the Omniscient Shell:

- **Respects** all existing build scripts (`build.psm1`, `tools/ci.psm1`)
- **Does NOT modify** PowerShell build processes
- **Can be built independently** using Cargo
- **Provides optional integration** for enhanced development workflows

## Testing Integration

The Omniscient Shell can be tested alongside PowerShell tests:

```bash
# Test PowerShell (existing)
Import-Module ./build.psm1
Start-PSPester

# Test Omniscient Shell (new)
cd omniscient-shell
cargo test
```

## Deployment

The Omniscient Shell is deployed as a standalone binary:

```
/usr/local/bin/omniscient-shell  # or C:\Program Files\OmniscientShell\
/usr/local/bin/omni              # short alias
```

It does NOT replace or modify PowerShell installations.

## Compatibility

- **Windows**: PowerShell 5.1+ or PowerShell 7+
- **Linux**: PowerShell 7+ (pwsh)
- **macOS**: PowerShell 7+ (pwsh)

All platforms use the same binary with platform-specific adaptations handled internally.

## Security Considerations

Following the security model from `.github/BUILD-INSTRUCTIONS.md`:

1. **No elevation required**: Runs with user permissions
2. **Sandboxed agents**: WASM and native isolation (Phase 2)
3. **Explicit consent**: All capabilities require user approval (Phase 2)
4. **Audit logging**: All actions logged to event ledger (Phase 2)
5. **Token security**: OAuth tokens in OS keychain (Phase 3)

## Future Enhancements

### Phase 2: Agent Integration
- PowerShell can invoke AI agents
- Agents can execute PowerShell commands (with consent)
- Bidirectional communication via event protocol

### Phase 3: OAuth Integration
- PowerShell commands can use OAuth-authenticated APIs
- Tokens managed securely by vault
- No secrets exposed to scripts

### Phase 4: Workspace Integration
- PowerShell scripts auto-save artifacts
- Retention policies enforce cleanup
- Workspace-aware command completion

## Troubleshooting

### PowerShell Not Found

```
Error: PowerShell not found. Please install PowerShell 7+ (pwsh)
Hint: Visit https://github.com/PowerShell/PowerShell/releases
```

Install PowerShell from official sources.

### Permission Denied

The shell requires read/write access to:
- `~/.omniscient/` - Configuration and state
- Current working directory - For workspace operations

### Integration Issues

Enable debug logging:
```bash
RUST_LOG=debug omniscient-shell
```

## Contributing

See [CONTRIBUTING.md](../.github/CONTRIBUTING.md) for development guidelines.

Modifications to PowerShell integration should:
1. Not break existing PowerShell functionality
2. Follow PowerShell best practices
3. Use proper error handling
4. Include tests
5. Document changes
