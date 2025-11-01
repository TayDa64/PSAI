#![allow(dead_code)]
//! Command palette for interactive commands

use std::collections::HashMap;

/// Command definition
#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub aliases: Vec<String>,
    pub handler: CommandHandler,
}

/// Command handler type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandHandler {
    WorkspaceSelect,
    WorkspaceClear,
    AgentList,
    AgentEnable,
    AgentDisable,
    ConfigReload,
    ConfigEdit,
    OAuthConnect,
    OAuthRevoke,
    VaultLock,
    VaultUnlock,
    ThemeSwitch,
    LayoutSwitch,
    Help,
    Quit,
}

/// Command palette for quick actions
pub struct CommandPalette {
    commands: HashMap<String, Command>,
}

impl CommandPalette {
    pub fn new() -> Self {
        let mut palette = CommandPalette {
            commands: HashMap::new(),
        };

        palette.register_default_commands();
        palette
    }

    fn register_default_commands(&mut self) {
        // Workspace commands
        self.register(Command {
            name: "workspace:select".to_string(),
            description: "Select a workspace directory".to_string(),
            aliases: vec!["ws:select".to_string(), "ws".to_string()],
            handler: CommandHandler::WorkspaceSelect,
        });

        self.register(Command {
            name: "workspace:clear".to_string(),
            description: "Clear workspace selection".to_string(),
            aliases: vec!["ws:clear".to_string()],
            handler: CommandHandler::WorkspaceClear,
        });

        // Agent commands
        self.register(Command {
            name: "agent:list".to_string(),
            description: "List all registered agents".to_string(),
            aliases: vec!["agents".to_string()],
            handler: CommandHandler::AgentList,
        });

        self.register(Command {
            name: "agent:enable".to_string(),
            description: "Enable an agent".to_string(),
            aliases: vec!["agent:on".to_string()],
            handler: CommandHandler::AgentEnable,
        });

        self.register(Command {
            name: "agent:disable".to_string(),
            description: "Disable an agent".to_string(),
            aliases: vec!["agent:off".to_string()],
            handler: CommandHandler::AgentDisable,
        });

        // Config commands
        self.register(Command {
            name: "config:reload".to_string(),
            description: "Reload configuration from disk".to_string(),
            aliases: vec!["reload".to_string()],
            handler: CommandHandler::ConfigReload,
        });

        self.register(Command {
            name: "config:edit".to_string(),
            description: "Open configuration in editor".to_string(),
            aliases: vec!["edit".to_string()],
            handler: CommandHandler::ConfigEdit,
        });

        // OAuth commands
        self.register(Command {
            name: "oauth:connect".to_string(),
            description: "Connect to OAuth provider".to_string(),
            aliases: vec!["connect".to_string()],
            handler: CommandHandler::OAuthConnect,
        });

        self.register(Command {
            name: "oauth:revoke".to_string(),
            description: "Revoke OAuth token".to_string(),
            aliases: vec!["revoke".to_string()],
            handler: CommandHandler::OAuthRevoke,
        });

        // Vault commands
        self.register(Command {
            name: "vault:lock".to_string(),
            description: "Lock the token vault".to_string(),
            aliases: vec!["lock".to_string()],
            handler: CommandHandler::VaultLock,
        });

        self.register(Command {
            name: "vault:unlock".to_string(),
            description: "Unlock the token vault".to_string(),
            aliases: vec!["unlock".to_string()],
            handler: CommandHandler::VaultUnlock,
        });

        // UI commands
        self.register(Command {
            name: "theme:switch".to_string(),
            description: "Switch color theme".to_string(),
            aliases: vec!["theme".to_string()],
            handler: CommandHandler::ThemeSwitch,
        });

        self.register(Command {
            name: "layout:switch".to_string(),
            description: "Switch layout preset".to_string(),
            aliases: vec!["layout".to_string()],
            handler: CommandHandler::LayoutSwitch,
        });

        // System commands
        self.register(Command {
            name: "help".to_string(),
            description: "Show help information".to_string(),
            aliases: vec!["?".to_string()],
            handler: CommandHandler::Help,
        });

        self.register(Command {
            name: "quit".to_string(),
            description: "Quit the application".to_string(),
            aliases: vec!["q".to_string(), "exit".to_string()],
            handler: CommandHandler::Quit,
        });
    }

    /// Register a command
    pub fn register(&mut self, command: Command) {
        self.commands.insert(command.name.clone(), command.clone());

        for alias in &command.aliases {
            self.commands.insert(alias.clone(), command.clone());
        }
    }

    /// Search for commands matching a query
    pub fn search(&self, query: &str) -> Vec<&Command> {
        let query_lower = query.to_lowercase();

        let mut results: Vec<&Command> = self
            .commands
            .values()
            .filter(|cmd| {
                cmd.name.to_lowercase().contains(&query_lower)
                    || cmd.description.to_lowercase().contains(&query_lower)
                    || cmd
                        .aliases
                        .iter()
                        .any(|a| a.to_lowercase().contains(&query_lower))
            })
            .collect();

        // Remove duplicates (from aliases)
        results.sort_by_key(|cmd| &cmd.name);
        results.dedup_by_key(|cmd| &cmd.name);

        results
    }

    /// Get command by name or alias
    pub fn get(&self, name: &str) -> Option<&Command> {
        self.commands.get(name)
    }

    /// Get all commands
    #[allow(dead_code)]
    pub fn all_commands(&self) -> Vec<&Command> {
        let mut commands: Vec<&Command> = self.commands.values().collect();
        commands.sort_by_key(|cmd| &cmd.name);
        commands.dedup_by_key(|cmd| &cmd.name);
        commands
    }
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_palette() {
        let palette = CommandPalette::new();

        // Test search
        let results = palette.search("workspace");
        assert!(results.len() >= 2);

        // Test get by name
        let cmd = palette.get("workspace:select");
        assert!(cmd.is_some());
        assert_eq!(cmd.unwrap().handler, CommandHandler::WorkspaceSelect);

        // Test get by alias
        let cmd = palette.get("ws");
        assert!(cmd.is_some());
        assert_eq!(cmd.unwrap().handler, CommandHandler::WorkspaceSelect);
    }

    #[test]
    fn test_search() {
        let palette = CommandPalette::new();

        let results = palette.search("oauth");
        assert!(!results.is_empty());

        let results = palette.search("vault");
        assert!(!results.is_empty());
    }
}
