//! Platform-specific abstractions

pub mod process;
pub mod filesystem;
pub mod sandbox;

#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub mod unix;

// Cross-platform abstractions
