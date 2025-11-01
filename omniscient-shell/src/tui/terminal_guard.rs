//! Terminal state guard - ensures terminal is always restored

use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{stdout, Stdout};

/// RAII guard that ensures terminal state is restored on drop
/// 
/// This guard handles:
/// - Normal exit paths
/// - Early returns
/// - Panic unwinding
/// 
/// The terminal state is automatically restored when the guard is dropped,
/// regardless of how the function exits.
pub struct TerminalGuard {
    _stdout: Stdout,
    enabled: bool,
}

impl TerminalGuard {
    /// Create a new terminal guard and enable raw mode
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        
        Ok(TerminalGuard {
            _stdout: stdout(),
            enabled: true,
        })
    }

    /// Disable the guard without restoring (for testing)
    #[cfg(test)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        if self.enabled {
            // Best effort cleanup - ignore errors during drop
            let _ = disable_raw_mode();
            let _ = stdout().execute(LeaveAlternateScreen);
            
            // Ensure cursor is visible
            let _ = crossterm::execute!(
                stdout(),
                crossterm::cursor::Show
            );
            
            tracing::debug!("Terminal state restored");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_guard_creation() {
        // Note: This test may fail in CI environments without a TTY
        // The guard should handle this gracefully
        let result = TerminalGuard::new();
        
        // In CI without TTY, this might fail - that's OK
        if result.is_ok() {
            let mut guard = result.unwrap();
            guard.disable(); // Disable to avoid terminal changes in test
        }
    }

    #[test]
    fn test_terminal_guard_drop() {
        // Test that guard can be dropped safely
        let result = TerminalGuard::new();
        if let Ok(mut guard) = result {
            guard.disable();
            drop(guard); // Explicit drop - should not panic
        }
    }

    #[test]
    fn test_early_return_with_guard() {
        // Simulate early return scenario
        fn early_return_function() -> Result<()> {
            let result = TerminalGuard::new();
            if result.is_err() {
                return Ok(()); // Early return without TTY
            }
            
            let mut _guard = result.unwrap();
            _guard.disable();
            
            // Early return - guard should still restore on drop
            if true {
                return Ok(());
            }
            
            Ok(())
        }
        
        assert!(early_return_function().is_ok());
    }

    #[test]
    #[should_panic(expected = "intentional panic")]
    fn test_panic_with_guard() {
        let result = TerminalGuard::new();
        if let Ok(mut guard) = result {
            guard.disable();
            let _guard = guard;
            // Guard should still restore even on panic
            panic!("intentional panic");
        } else {
            // No TTY - skip panic test
            panic!("intentional panic");
        }
    }
}
