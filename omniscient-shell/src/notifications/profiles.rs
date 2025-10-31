//! Notification system profiles

use serde::{Deserialize, Serialize};

/// Notification profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationProfile {
    pub name: String,
    pub enabled_channels: Vec<String>,
    pub priority_threshold: Priority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl NotificationProfile {
    /// Minimal profile (default)
    pub fn minimal() -> Self {
        NotificationProfile {
            name: "minimal".to_string(),
            enabled_channels: vec!["tui".to_string()],
            priority_threshold: Priority::Warning,
        }
    }

    /// Verbose profile
    pub fn verbose() -> Self {
        NotificationProfile {
            name: "verbose".to_string(),
            enabled_channels: vec!["tui".to_string(), "system".to_string()],
            priority_threshold: Priority::Info,
        }
    }

    /// Silent profile
    pub fn silent() -> Self {
        NotificationProfile {
            name: "silent".to_string(),
            enabled_channels: vec![],
            priority_threshold: Priority::Critical,
        }
    }

    /// Check if a notification should be shown
    pub fn should_notify(&self, priority: Priority) -> bool {
        priority >= self.priority_threshold
    }
}

impl Default for NotificationProfile {
    fn default() -> Self {
        Self::minimal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiles() {
        let minimal = NotificationProfile::minimal();
        assert!(minimal.should_notify(Priority::Warning));
        assert!(!minimal.should_notify(Priority::Info));

        let verbose = NotificationProfile::verbose();
        assert!(verbose.should_notify(Priority::Info));
    }
}
