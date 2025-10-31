//! Workspace and artifact management (Phase 4)

pub mod selection;
pub mod artifacts;
pub mod retention;

pub use selection::Workspace;
pub use artifacts::Artifact;
pub use retention::{RetentionPolicy, PruneStrategy};
