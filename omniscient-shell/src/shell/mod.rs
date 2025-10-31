//! PowerShell integration layer

pub mod integration;
pub mod command_router;
pub mod process_supervision;
pub mod history;

pub use integration::PowerShellIntegration;
