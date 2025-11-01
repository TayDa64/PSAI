//! Error types with recovery hints

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OmniError {
    #[error("Configuration error: {message}")]
    Config {
        message: String,
        hint: Option<String>,
        recovery: RecoveryAction,
    },

    #[error("Graphics backend error: {message}")]
    Graphics {
        message: String,
        hint: Option<String>,
        recovery: RecoveryAction,
    },

    #[error("PowerShell integration error: {message}")]
    Shell {
        message: String,
        hint: Option<String>,
        recovery: RecoveryAction,
    },

    #[error("Agent error: {message}")]
    Agent {
        message: String,
        hint: Option<String>,
        recovery: RecoveryAction,
    },

    #[error("OAuth error: {message}")]
    OAuth {
        message: String,
        hint: Option<String>,
        recovery: RecoveryAction,
    },

    #[error("Workspace error: {message}")]
    Workspace {
        message: String,
        hint: Option<String>,
        recovery: RecoveryAction,
    },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// Recovery actions for errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryAction {
    /// Retry the operation
    Retry,
    /// Fall back to alternative implementation
    Fallback(String),
    /// Prompt user for input/action
    PromptUser(String),
    /// Automatically fix the issue
    AutoFix(String),
    /// No automatic recovery possible
    None,
}

impl OmniError {
    pub fn config(
        message: impl Into<String>,
        hint: impl Into<Option<String>>,
        recovery: RecoveryAction,
    ) -> Self {
        OmniError::Config {
            message: message.into(),
            hint: hint.into(),
            recovery,
        }
    }

    pub fn graphics(
        message: impl Into<String>,
        hint: impl Into<Option<String>>,
        recovery: RecoveryAction,
    ) -> Self {
        OmniError::Graphics {
            message: message.into(),
            hint: hint.into(),
            recovery,
        }
    }

    pub fn shell(
        message: impl Into<String>,
        hint: impl Into<Option<String>>,
        recovery: RecoveryAction,
    ) -> Self {
        OmniError::Shell {
            message: message.into(),
            hint: hint.into(),
            recovery,
        }
    }

    pub fn agent(
        message: impl Into<String>,
        hint: impl Into<Option<String>>,
        recovery: RecoveryAction,
    ) -> Self {
        OmniError::Agent {
            message: message.into(),
            hint: hint.into(),
            recovery,
        }
    }

    pub fn oauth(
        message: impl Into<String>,
        hint: impl Into<Option<String>>,
        recovery: RecoveryAction,
    ) -> Self {
        OmniError::OAuth {
            message: message.into(),
            hint: hint.into(),
            recovery,
        }
    }

    pub fn workspace(
        message: impl Into<String>,
        hint: impl Into<Option<String>>,
        recovery: RecoveryAction,
    ) -> Self {
        OmniError::Workspace {
            message: message.into(),
            hint: hint.into(),
            recovery,
        }
    }

    pub fn hint(&self) -> Option<&str> {
        match self {
            OmniError::Config { hint, .. } => hint.as_deref(),
            OmniError::Graphics { hint, .. } => hint.as_deref(),
            OmniError::Shell { hint, .. } => hint.as_deref(),
            OmniError::Agent { hint, .. } => hint.as_deref(),
            OmniError::OAuth { hint, .. } => hint.as_deref(),
            OmniError::Workspace { hint, .. } => hint.as_deref(),
            _ => None,
        }
    }

    pub fn recovery_action(&self) -> RecoveryAction {
        match self {
            OmniError::Config { recovery, .. } => recovery.clone(),
            OmniError::Graphics { recovery, .. } => recovery.clone(),
            OmniError::Shell { recovery, .. } => recovery.clone(),
            OmniError::Agent { recovery, .. } => recovery.clone(),
            OmniError::OAuth { recovery, .. } => recovery.clone(),
            OmniError::Workspace { recovery, .. } => recovery.clone(),
            _ => RecoveryAction::None,
        }
    }

    /// Get a user-friendly error message with recovery suggestion
    pub fn display_with_recovery(&self) -> String {
        let mut msg = format!("{}", self);

        if let Some(hint) = self.hint() {
            msg.push_str(&format!("\n💡 Hint: {}", hint));
        }

        match self.recovery_action() {
            RecoveryAction::Retry => {
                msg.push_str("\n🔄 Recovery: Retry the operation");
            }
            RecoveryAction::Fallback(alt) => {
                msg.push_str(&format!("\n🔄 Recovery: Falling back to {}", alt));
            }
            RecoveryAction::PromptUser(prompt) => {
                msg.push_str(&format!("\n❓ Action needed: {}", prompt));
            }
            RecoveryAction::AutoFix(action) => {
                msg.push_str(&format!("\n🔧 Auto-fixing: {}", action));
            }
            RecoveryAction::None => {}
        }

        msg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_with_recovery() {
        let err = OmniError::config(
            "Invalid config version",
            Some("Update to schema v0.1".to_string()),
            RecoveryAction::AutoFix("Migrating config to v0.1".to_string()),
        );

        let display = err.display_with_recovery();
        assert!(display.contains("Invalid config version"));
        assert!(display.contains("Update to schema v0.1"));
        assert!(display.contains("Migrating config to v0.1"));
    }

    #[test]
    fn test_recovery_action() {
        let err = OmniError::graphics(
            "Notcurses not available",
            Some("Install notcurses library".to_string()),
            RecoveryAction::Fallback("Kitty protocol".to_string()),
        );

        assert_eq!(
            err.recovery_action(),
            RecoveryAction::Fallback("Kitty protocol".to_string())
        );
    }
}
