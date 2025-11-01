//! Agent runtime and registry (Phase 2)

pub mod capabilities;
pub mod event_protocol;
pub mod manifest;
pub mod native_runner;
pub mod registry;
pub mod runtime;
pub mod wasm_host;

pub use capabilities::{Capability, CapabilityManager};
pub use event_protocol::Event;
pub use manifest::Manifest;
pub use registry::AgentRegistry;
pub use runtime::AgentRuntime;
