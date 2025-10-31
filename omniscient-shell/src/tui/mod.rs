//! TUI (Terminal User Interface) module

pub mod dashboard;
pub mod panes;
pub mod cards;
pub mod theme;
pub mod layout;
pub mod command_palette;

pub use dashboard::Dashboard;
pub use command_palette::{CommandPalette, Command, CommandHandler};
