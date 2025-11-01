//! Notification channels

use crate::notifications::profiles::Priority;
use anyhow::Result;

/// Notification message
#[derive(Debug, Clone)]
pub struct Notification {
    pub title: String,
    pub message: String,
    pub priority: Priority,
}

/// Notification channel trait
pub trait NotificationChannel: Send + Sync {
    fn send(&self, notification: &Notification) -> Result<()>;
    fn name(&self) -> &str;
}

/// TUI channel (in-app notifications)
pub struct TuiChannel;

impl NotificationChannel for TuiChannel {
    fn send(&self, notification: &Notification) -> Result<()> {
        // In a real implementation, this would add to the log pane
        tracing::info!("[TUI] {}: {}", notification.title, notification.message);
        Ok(())
    }

    fn name(&self) -> &str {
        "tui"
    }
}

/// System channel (OS notifications)
pub struct SystemChannel;

impl NotificationChannel for SystemChannel {
    fn send(&self, notification: &Notification) -> Result<()> {
        // In a real implementation, this would use system notifications
        // - Windows: Windows notifications API
        // - macOS: AppleScript / terminal-notifier
        // - Linux: notify-send
        tracing::info!("[System] {}: {}", notification.title, notification.message);
        Ok(())
    }

    fn name(&self) -> &str {
        "system"
    }
}
