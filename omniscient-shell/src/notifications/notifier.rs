//! Notification system

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

use crate::notifications::profiles::{NotificationProfile, Priority};
use crate::notifications::channels::{Notification, NotificationChannel, TuiChannel, SystemChannel};

/// Notification dispatcher
pub struct Notifier {
    profile: Arc<NotificationProfile>,
    channels: HashMap<String, Box<dyn NotificationChannel>>,
}

impl Notifier {
    pub fn new(profile: NotificationProfile) -> Self {
        let mut channels: HashMap<String, Box<dyn NotificationChannel>> = HashMap::new();
        channels.insert("tui".to_string(), Box::new(TuiChannel));
        channels.insert("system".to_string(), Box::new(SystemChannel));

        Notifier {
            profile: Arc::new(profile),
            channels,
        }
    }

    /// Send a notification
    pub fn notify(&self, title: impl Into<String>, message: impl Into<String>, priority: Priority) -> Result<()> {
        if !self.profile.should_notify(priority) {
            return Ok(());
        }

        let notification = Notification {
            title: title.into(),
            message: message.into(),
            priority,
        };

        // Send to all enabled channels
        for channel_name in &self.profile.enabled_channels {
            if let Some(channel) = self.channels.get(channel_name) {
                channel.send(&notification)?;
            }
        }

        Ok(())
    }

    /// Update notification profile
    pub fn set_profile(&mut self, profile: NotificationProfile) {
        self.profile = Arc::new(profile);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notifier() {
        let profile = NotificationProfile::minimal();
        let notifier = Notifier::new(profile);

        // Should notify (warning >= warning threshold)
        let result = notifier.notify("Test", "Message", Priority::Warning);
        assert!(result.is_ok());

        // Should not notify (info < warning threshold)
        let result = notifier.notify("Test", "Message", Priority::Info);
        assert!(result.is_ok()); // Still ok, just doesn't send
    }
}
