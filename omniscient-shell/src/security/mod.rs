//! Security and sandboxing (Phase 2)

pub mod capabilities;
pub mod consent;
pub mod isolation;
pub mod resource_limits;

// Re-export agent capabilities for security module
pub use crate::agents::capabilities::{Capability, CapabilityManager};
pub use crate::oauth::consent::ConsentLedger;
