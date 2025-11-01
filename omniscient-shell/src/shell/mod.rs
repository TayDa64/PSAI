//! PowerShell integration layer

pub mod command_router;
pub mod history;
pub mod integration;
pub mod process_supervision;

pub use integration::PowerShellIntegration;
