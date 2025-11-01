#![allow(dead_code)]
//! PowerShell integration implementation

use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

/// PowerShell integration layer
pub struct PowerShellIntegration {
    pwsh_path: String,
    history: Arc<Mutex<Vec<String>>>,
}

impl PowerShellIntegration {
    /// Create a new PowerShell integration instance
    pub fn new() -> Result<Self> {
        let pwsh_path = Self::find_powershell().context("Failed to find PowerShell executable")?;

        Ok(PowerShellIntegration {
            pwsh_path,
            history: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Find PowerShell executable on the system
    fn find_powershell() -> Result<String> {
        // Try pwsh first (PowerShell 7+)
        if let Ok(output) = Command::new("pwsh")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
        {
            if output.success() {
                return Ok("pwsh".to_string());
            }
        }

        // Fall back to powershell (Windows PowerShell)
        if cfg!(windows) {
            if let Ok(output) = Command::new("powershell")
                .arg("-Version")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
            {
                if output.success() {
                    return Ok("powershell".to_string());
                }
            }
        }

        anyhow::bail!("PowerShell not found. Please install PowerShell 7+ (pwsh)")
    }

    /// Execute a PowerShell command
    pub async fn execute(&self, command: &str) -> Result<String> {
        let mut history = self.history.lock().await;
        history.push(command.to_string());

        let output = Command::new(&self.pwsh_path)
            .arg("-NoProfile")
            .arg("-NonInteractive")
            .arg("-Command")
            .arg(command)
            .output()
            .context("Failed to execute PowerShell command")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("PowerShell command failed: {}", stderr)
        }
    }

    /// Get command history
    pub async fn get_history(&self) -> Vec<String> {
        let history = self.history.lock().await;
        history.clone()
    }

    /// Get PowerShell version
    pub async fn get_version(&self) -> Result<String> {
        self.execute("$PSVersionTable.PSVersion.ToString()").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_powershell_integration() {
        if let Ok(ps) = PowerShellIntegration::new() {
            // Basic echo test
            let result = ps.execute("Write-Output 'Hello'").await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_get_version() {
        if let Ok(ps) = PowerShellIntegration::new() {
            let version = ps.get_version().await;
            assert!(version.is_ok());
        }
    }
}
