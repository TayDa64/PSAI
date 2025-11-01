//! Command history management

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct History {
    commands: Arc<Mutex<Vec<String>>>,
    max_size: usize,
}

impl History {
    pub fn new(max_size: usize) -> Self {
        History {
            commands: Arc::new(Mutex::new(Vec::new())),
            max_size,
        }
    }

    pub async fn add(&self, command: String) {
        let mut commands = self.commands.lock().await;
        commands.push(command);

        // Trim to max size
        if commands.len() > self.max_size {
            commands.remove(0);
        }
    }

    pub async fn get_all(&self) -> Vec<String> {
        let commands = self.commands.lock().await;
        commands.clone()
    }

    pub async fn search(&self, query: &str) -> Vec<String> {
        let commands = self.commands.lock().await;
        commands
            .iter()
            .filter(|cmd| cmd.contains(query))
            .cloned()
            .collect()
    }
}
