//! Process supervision for PowerShell instances

use std::process::Child;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ProcessSupervisor {
    processes: Arc<Mutex<Vec<Child>>>,
}

impl ProcessSupervisor {
    pub fn new() -> Self {
        ProcessSupervisor {
            processes: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add_process(&self, child: Child) {
        let mut processes = self.processes.lock().await;
        processes.push(child);
    }

    pub async fn cleanup(&self) {
        let mut processes = self.processes.lock().await;
        for child in processes.iter_mut() {
            let _ = child.kill();
        }
        processes.clear();
    }
}
