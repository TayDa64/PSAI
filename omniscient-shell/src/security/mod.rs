//! Security and sandboxing (Phase 2)

// Re-export agent capabilities for security module
#[cfg(feature = "omniscience")]
pub use crate::agents::capabilities::{Capability, CapabilityManager};
#[cfg(feature = "omniscience")]
pub use crate::oauth::consent::ConsentLedger;
