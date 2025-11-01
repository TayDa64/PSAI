//! Platform-specific abstractions

#![allow(dead_code)]

pub mod filesystem;
pub mod process;
pub mod sandbox;

#[cfg(windows)]
pub mod windows {
    // Windows-specific implementations
}

#[cfg(unix)]
pub mod unix {
    // Unix-specific implementations
}
