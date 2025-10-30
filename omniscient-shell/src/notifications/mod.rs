//! Notification system (Phase 5)

pub mod notifier;
pub mod profiles;
pub mod channels;

pub use notifier::Notifier;
pub use profiles::{NotificationProfile, Priority};
pub use channels::{Notification, NotificationChannel};
