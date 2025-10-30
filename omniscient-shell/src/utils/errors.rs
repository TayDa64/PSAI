//! Error types with recovery hints

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OmniError {
    #[error("Configuration error: {message}")]
    Config {
        message: String,
        hint: Option<String>,
    },

    #[error("Graphics backend error: {message}")]
    Graphics {
        message: String,
        hint: Option<String>,
    },

    #[error("PowerShell integration error: {message}")]
    Shell {
        message: String,
        hint: Option<String>,
    },

    #[error("Agent error: {message}")]
    Agent {
        message: String,
        hint: Option<String>,
    },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl OmniError {
    pub fn config(message: impl Into<String>, hint: impl Into<Option<String>>) -> Self {
        OmniError::Config {
            message: message.into(),
            hint: hint.into(),
        }
    }

    pub fn graphics(message: impl Into<String>, hint: impl Into<Option<String>>) -> Self {
        OmniError::Graphics {
            message: message.into(),
            hint: hint.into(),
        }
    }

    pub fn shell(message: impl Into<String>, hint: impl Into<Option<String>>) -> Self {
        OmniError::Shell {
            message: message.into(),
            hint: hint.into(),
        }
    }

    pub fn agent(message: impl Into<String>, hint: impl Into<Option<String>>) -> Self {
        OmniError::Agent {
            message: message.into(),
            hint: hint.into(),
        }
    }

    pub fn hint(&self) -> Option<&str> {
        match self {
            OmniError::Config { hint, .. } => hint.as_deref(),
            OmniError::Graphics { hint, .. } => hint.as_deref(),
            OmniError::Shell { hint, .. } => hint.as_deref(),
            OmniError::Agent { hint, .. } => hint.as_deref(),
            _ => None,
        }
    }
}
