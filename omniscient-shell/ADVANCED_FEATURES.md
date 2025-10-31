# Advanced Features - Omniscient Shell

## Overview

This document describes the advanced features added to the Omniscient Shell beyond the initial 5-phase implementation.

## 1. CI/CD Pipeline

**Location:** `.github/workflows/omniscient-ci.yml`

### Features
- **Multi-platform testing:** Linux, Windows, macOS
- **Multiple test jobs:**
  - Formatting checks (`cargo fmt`)
  - Linting (`cargo clippy`)
  - Unit tests (minimal + WASM features)
  - Release builds for all platforms
  - Security audit (`cargo-audit`)
  - Code coverage (`cargo-llvm-cov`)

### Artifacts
- Release binaries for each platform
- Code coverage reports (uploaded to Codecov)

### Triggers
- Push to main, develop, or copilot branches
- Pull requests to main or develop

## 2. Structured Error Recovery

**Location:** `src/utils/errors.rs`

### Enhancement
Extended error types with recovery actions:

```rust
pub enum RecoveryAction {
    Retry,                      // Retry the operation
    Fallback(String),           // Fall back to alternative
    PromptUser(String),         // Request user input
    AutoFix(String),            // Automatically fix
    None,                       // No recovery available
}
```

### Features
- **Automatic recovery hints:** Each error includes suggested recovery action
- **User-friendly display:** `display_with_recovery()` formats errors with recovery steps
- **Context-aware:** Different recovery strategies based on error type

### Example
```rust
let err = OmniError::graphics(
    "Notcurses not available",
    Some("Install notcurses library".to_string()),
    RecoveryAction::Fallback("Kitty protocol".to_string()),
);

println!("{}", err.display_with_recovery());
// Output:
// Graphics backend error: Notcurses not available
// 💡 Hint: Install notcurses library
// 🔄 Recovery: Falling back to Kitty protocol
```

## 3. Telemetry System

**Location:** `src/utils/telemetry.rs`

### Design Principles
- **Opt-in only:** Disabled by default
- **Performance-only:** Tracks operation durations and success rates
- **No secrets:** Automatically sanitizes sensitive data
- **Privacy-first:** Can be disabled at any time, clears all data on disable

### Features
- **Performance metrics:** Track operation duration with metadata
- **Event recording:** Record success/failure of operations
- **Sampling:** Configurable sample rate (0.0 - 1.0)
- **Summary statistics:** Get aggregated metrics
- **Automatic sanitization:** Removes token, password, secret, key, auth, credential fields

### Usage
```rust
let telemetry = TelemetryCollector::new(config);

// Record an event
telemetry.record_event("agent_execution", Some(150), metadata, true).await?;

// Track performance
let metric = PerformanceMetric::new("workspace_selection")
    .with_metadata("type", "explicit");
tokio::time::sleep(Duration::from_millis(10)).await;
telemetry.record_performance(metric, true).await?;

// Get summary
let summary = telemetry.get_summary().await;
```

### Configuration
```toml
[telemetry]
enabled = false  # Opt-in only
sample_rate = 1.0
endpoint = "https://telemetry.example.com/api/events"
```

## 4. Command Palette

**Location:** `src/tui/command_palette.rs`

### Features
- **Interactive command discovery:** Search commands by name, description, or alias
- **Quick actions:** Pre-configured commands for common operations
- **Keyboard shortcuts:** Access via hotkey (e.g., Ctrl+P)
- **Fuzzy search:** Find commands with partial matches

### Built-in Commands

#### Workspace Commands
- `workspace:select` (alias: `ws`, `ws:select`) - Select workspace directory
- `workspace:clear` (alias: `ws:clear`) - Clear workspace selection

#### Agent Commands
- `agent:list` (alias: `agents`) - List all registered agents
- `agent:enable` (alias: `agent:on`) - Enable an agent
- `agent:disable` (alias: `agent:off`) - Disable an agent

#### Config Commands
- `config:reload` (alias: `reload`) - Reload configuration
- `config:edit` (alias: `edit`) - Open config in editor

#### OAuth Commands
- `oauth:connect` (alias: `connect`) - Connect to OAuth provider
- `oauth:revoke` (alias: `revoke`) - Revoke OAuth token

#### Vault Commands
- `vault:lock` (alias: `lock`) - Lock token vault
- `vault:unlock` (alias: `unlock`) - Unlock token vault

#### UI Commands
- `theme:switch` (alias: `theme`) - Switch color theme
- `layout:switch` (alias: `layout`) - Switch layout preset

#### System Commands
- `help` (alias: `?`) - Show help
- `quit` (alias: `q`, `exit`) - Quit application

### Usage
```rust
let palette = CommandPalette::new();

// Search for commands
let results = palette.search("workspace");

// Execute command
if let Some(cmd) = palette.get("ws:select") {
    match cmd.handler {
        CommandHandler::WorkspaceSelect => { /* handle */ }
        _ => {}
    }
}
```

## 5. Schema Migration Tools

**Location:** `src/state/migrations.rs`

### Enhancement
Added comprehensive migration management:

```rust
// Check if migration needed
if needs_migration(&conn)? {
    migrate(&mut conn)?;
}

// Get current version
let version = current_version(&conn)?;

// Rollback (use with caution!)
rollback_to_version(&mut conn, 0)?;
```

### Features
- **Version tracking:** Maintains migration history in `schema_version` table
- **Automatic migration:** Runs missing migrations on startup
- **Rollback support:** Can revert to previous schema versions
- **Forward-compatible:** Easy to add new migrations

### Adding New Migrations
```rust
fn migrate_to_v2(conn: &mut Connection) -> Result<()> {
    tracing::info!("Migrating to schema version 2");
    
    // Add your migration SQL here
    conn.execute(
        "ALTER TABLE artifacts ADD COLUMN tags TEXT",
        [],
    )?;

    // Record migration
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    
    conn.execute(
        "INSERT INTO schema_version (version, applied_at) VALUES (?1, ?2)",
        [2, now as i32],
    )?;

    Ok(())
}
```

Then update `CURRENT_VERSION` and add to `migrate()`:
```rust
const CURRENT_VERSION: i32 = 2;

// In migrate function:
if version < 2 {
    migrate_to_v2(conn)?;
}
```

## 6. Security Audit

**Location:** `.github/workflows/omniscient-ci.yml`

### Features
- **Automated dependency scanning:** Uses `cargo-audit`
- **CI integration:** Runs on every push and PR
- **Vulnerability detection:** Identifies known security issues in dependencies
- **Advisory database:** Checks against RustSec Advisory Database

### Manual Usage
```bash
cargo install cargo-audit
cd omniscient-shell
cargo audit
```

## 7. Code Coverage

**Location:** `.github/workflows/omniscient-ci.yml`

### Features
- **LLVM-based coverage:** Uses `cargo-llvm-cov`
- **CI integration:** Generates coverage on every push
- **Codecov upload:** Automatically uploads to Codecov
- **LCOV format:** Compatible with standard coverage tools

### Manual Usage
```bash
cargo install cargo-llvm-cov
cd omniscient-shell
cargo llvm-cov --no-default-features --features kitty --lcov --output-path lcov.info
```

## Future Enhancements

### Planned
1. **Real-time Collaboration:** Multi-user workspace support
2. **Plugin Marketplace:** Downloadable agent extensions
3. **Advanced Media:** Full FFmpeg integration with hardware acceleration
4. **Performance Dashboard:** Real-time telemetry visualization in TUI
5. **Cloud Sync:** Workspace and config synchronization
6. **Voice Control:** Speech-to-command with wake word detection
7. **AI Assistant:** Built-in coding assistant using LLMs
8. **Graph Visualization:** Dependency and workflow graphs
9. **Custom Themes:** Theme editor with live preview
10. **Scripting Language:** Embedded scripting for automation

## Contributing

When adding new advanced features:

1. **Follow existing patterns:** Use the same error handling, telemetry, and testing approaches
2. **Add tests:** Every feature needs unit tests
3. **Update documentation:** Add to this file and README.md
4. **CI integration:** Add relevant checks to the workflow
5. **Opt-in for privacy:** User data features must be opt-in
6. **Security first:** Run security audit before committing

## References

- [Main README](README.md)
- [Implementation Guide](IMPLEMENTATION.md)
- [PowerShell Integration](INTEGRATION.md)
- [CI Workflow](.github/workflows/omniscient-ci.yml)
