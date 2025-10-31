//! Agent runtime and registry (Phase 2)

pub mod runtime;
pub mod registry;
pub mod manifest;
pub mod wasm_host;
pub mod native_runner;
pub mod event_protocol;
pub mod capabilities;

pub use runtime::AgentRuntime;
pub use registry::AgentRegistry;
pub use manifest::Manifest;
pub use capabilities::{Capability, CapabilityManager};
pub use event_protocol::Event;
