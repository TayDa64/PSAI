//! Platform-specific abstractions

pub mod process;
pub mod filesystem;
pub mod sandbox;

#[cfg(windows)]
pub mod windows {
    // Windows-specific implementations
}

#[cfg(unix)]
pub mod unix {
    // Unix-specific implementations
}

pub use process::ProcessManager;
pub use filesystem::FileSystem;
pub use sandbox::{SandboxConfig, apply_sandbox};
