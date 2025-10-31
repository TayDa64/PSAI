# PowerShell AI Coding Agent Instructions

## Architecture Overview

PowerShell is organized into distinct functional layers:

- **System.Management.Automation**: Core PowerShell engine containing cmdlet execution, pipeline, parser, and runtime
- **Microsoft.PowerShell.Commands.***: Command implementations (Management, Utility, Diagnostics, Security)  
- **Microsoft.PowerShell.ConsoleHost**: Interactive shell host implementation
- **powershell-win-core/powershell-unix**: Platform-specific entry points and bootstrapping
- **src/Modules**: PowerShell modules with platform-specific variants (Shared/Windows/Unix)

## Essential Build & Development Workflow

### Bootstrap Environment
```powershell
# Essential first step - sets up .NET SDK, dependencies
Import-Module ./build.psm1
Start-PSBootstrap -Scenario Both  # For full environment
Start-PSBootstrap -Scenario Dotnet # For minimal .NET only
```

### Build Commands
```powershell
# Standard development build
Start-PSBuild

# Clean build with output directory
Start-PSBuild -Clean -Output ./debug

# Cross-platform builds
Start-PSBuild -Runtime linux-x64
Start-PSBuild -Runtime osx-x64 
Start-PSBuild -Runtime win-x64
```

### Testing
```powershell
# Run all Pester tests (primary test suite)
Start-PSPester

# Run specific test patterns
Start-PSPester -Tests "SomeTestSuite*"

# Run xUnit tests (.NET unit tests)  
Start-PSxUnit

# Always use -noprofile when launching test PowerShell processes
$powershell = Join-Path -Path $PsHome -ChildPath "pwsh"
& $powershell -noprofile -command "Test-Something"
```

## Critical File Patterns

### Project Structure
- **global.json**: Pins .NET SDK version (currently 10.0.100-rc.2.25502.107)
- **PowerShell.Common.props**: Shared MSBuild properties across all projects
- **DotnetRuntimeMetadata.json**: Runtime feed and version metadata for releases
- **build.psm1**: Primary build orchestration with 4000+ lines of build logic

### Source Organization
- **src/System.Management.Automation/engine/**: Core PowerShell execution engine
- **src/System.Management.Automation/FormatAndOutput/**: PowerShell formatting system
- **src/Microsoft.PowerShell.Commands.*/**: Cmdlet implementations by functional area
- **test/powershell/**: Pester tests organized by module/functionality
- **test/xUnit/**: C# unit tests for core engine components

### Platform Handling
Platform-specific code uses these patterns:
```csharp
// In C# projects
#if WINDOWS
// Windows-specific implementation
#elif UNIX  
// Unix-specific implementation
#endif
```

PowerShell modules have platform directories:
- `src/Modules/Shared/`: Cross-platform modules
- `src/Modules/Windows/`: Windows-only modules  
- `src/Modules/Unix/`: Unix-only modules

## Development Conventions

### Error Handling in Tests
Use `FullyQualifiedErrorId` for reliable error checking:
```powershell
{ Get-Item "NonExistentFile" -ErrorAction Stop } | 
    Should -Throw -ErrorId "PathNotFound,Microsoft.PowerShell.Commands.GetItemCommand"
```

### Module Development
- `.psd1` files often differ between platforms due to different `CmdletsToExport`
- Content files (ps1xml, psm1, psd1, ps1) are copied as-is by dotnet build
- Test modules are auto-loaded from `test/tools/Modules` via `$env:PSModulePath`

### Git and Versioning
```powershell
# Sync tags for version operations (critical for release builds)
Sync-PSTags -AddRemoteIfMissing

# Get version information
Get-PSVersion
Get-PSLatestTag  
```

### CI/Build Integration
- Use `Test-DailyBuild` to detect scheduled builds vs PR builds
- `tools/ci.psm1` contains CI-specific functions that import build.psm1
- Background processes in tests require careful cleanup and process ID tracking

## Common Gotchas

1. **Always import build.psm1 first**: `Import-Module ./build.psm1` before other build operations
2. **Platform detection**: Use `$IsWindows`, `$IsLinux`, `$IsMacOS` variables consistently
3. **Test isolation**: Tests must not depend on user profiles or customizations  
4. **Native dependencies**: libpsl-native provides cross-platform native functionality
5. **Module loading**: Some cmdlets are platform-specific and won't load on all systems

## Key Integration Points

- **CIM/WMI**: Windows Management via `Microsoft.Management.Infrastructure.CimCmdlets`
- **Event logging**: Platform-specific via `Microsoft.PowerShell.CoreCLR.Eventing`
- **Security**: Execution policy, code signing via `Microsoft.PowerShell.Security`
- **Remoting**: WSMan protocol via `Microsoft.WSMan.Management` and `Microsoft.WSMan.Runtime`

This codebase requires understanding both .NET/C# development and PowerShell scripting patterns. When modifying core engine code, consider cross-platform implications and test on multiple platforms.
