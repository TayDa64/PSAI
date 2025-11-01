#![allow(dead_code)]
//! Command router for PowerShell commands

use anyhow::Result;

pub struct CommandRouter {
    // Command routing logic
}

impl CommandRouter {
    pub fn new() -> Self {
        CommandRouter {}
    }

    /// Route a command to the appropriate handler
    /// TODO: Add support for agent-specific routing patterns
    /// TODO: Add configurable routing rules
    pub fn route(&self, command: &str) -> Result<RouteTarget> {
        // Simple routing logic - can be expanded
        if command.starts_with("omni:") {
            Ok(RouteTarget::OmniscientShell)
        } else if command.starts_with("@") {
            // Extract agent name from @agent-name syntax
            let agent_name = command
                .trim_start_matches('@')
                .split_whitespace()
                .next()
                .unwrap_or("default");
            Ok(RouteTarget::Agent(agent_name.to_string()))
        } else {
            Ok(RouteTarget::PowerShell)
        }
    }

    /// Check if a command should be routed to an agent
    pub fn is_agent_command(&self, command: &str) -> bool {
        command.starts_with('@') || command.starts_with("omni:")
    }

    /// Extract the command without routing prefix
    /// TODO: Add support for complex command parsing
    pub fn extract_command<'a>(&self, command: &'a str) -> &'a str {
        if command.starts_with("omni:") {
            command.trim_start_matches("omni:")
        } else if command.starts_with('@') {
            // Remove @agent-name prefix
            command
                .trim_start_matches('@')
                .split_once(' ')
                .map(|(_, cmd)| cmd)
                .unwrap_or("")
        } else {
            command
        }
    }
}

impl Default for CommandRouter {
    fn default() -> Self {
        Self::new()
    }
}

pub enum RouteTarget {
    PowerShell,
    OmniscientShell,
    Agent(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_router_creation() {
        let router = CommandRouter::new();
        assert!(std::mem::size_of_val(&router) == 0);
    }

    #[test]
    fn test_default_trait() {
        let router = CommandRouter::default();
        assert!(std::mem::size_of_val(&router) == 0);
    }

    #[test]
    fn test_route_powershell_command() {
        let router = CommandRouter::new();
        let result = router.route("Get-Process");
        assert!(result.is_ok());

        if let Ok(RouteTarget::PowerShell) = result {
            // Expected
        } else {
            panic!("Expected PowerShell route");
        }
    }

    #[test]
    fn test_route_omniscient_command() {
        let router = CommandRouter::new();
        let result = router.route("omni:help");
        assert!(result.is_ok());

        if let Ok(RouteTarget::OmniscientShell) = result {
            // Expected
        } else {
            panic!("Expected OmniscientShell route");
        }
    }

    #[test]
    fn test_route_agent_command() {
        let router = CommandRouter::new();
        let result = router.route("@myagent do something");
        assert!(result.is_ok());

        if let Ok(RouteTarget::Agent(name)) = result {
            assert_eq!(name, "myagent");
        } else {
            panic!("Expected Agent route");
        }
    }

    #[test]
    fn test_is_agent_command() {
        let router = CommandRouter::new();

        assert!(router.is_agent_command("@agent test"));
        assert!(router.is_agent_command("omni:test"));
        assert!(!router.is_agent_command("Get-Process"));
    }

    #[test]
    fn test_extract_command() {
        let router = CommandRouter::new();

        assert_eq!(router.extract_command("omni:help"), "help");
        assert_eq!(router.extract_command("@agent do task"), "do task");
        assert_eq!(router.extract_command("Get-Process"), "Get-Process");
    }

    #[test]
    fn test_extract_command_edge_cases() {
        let router = CommandRouter::new();

        assert_eq!(router.extract_command("omni:"), "");
        assert_eq!(router.extract_command("@agent"), "");
        assert_eq!(router.extract_command(""), "");
    }
}
