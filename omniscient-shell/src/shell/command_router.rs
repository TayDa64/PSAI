//! Command router for PowerShell commands

use anyhow::Result;

pub struct CommandRouter {
    // Command routing logic
}

impl CommandRouter {
    pub fn new() -> Self {
        CommandRouter {}
    }

    pub fn route(&self, command: &str) -> Result<RouteTarget> {
        // Simple routing logic - can be expanded
        if command.starts_with("omni:") {
            Ok(RouteTarget::OmniscientShell)
        } else {
            Ok(RouteTarget::PowerShell)
        }
    }
}

pub enum RouteTarget {
    PowerShell,
    OmniscientShell,
    Agent(String),
}
