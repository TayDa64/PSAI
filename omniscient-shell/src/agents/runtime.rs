//! Agent runtime orchestration

use anyhow::Result;
use std::sync::Arc;

use crate::agents::manifest::Manifest;
use crate::agents::capabilities::CapabilityManager;
use crate::agents::event_protocol::Event;
use crate::agents::wasm_host::WasmHost;
use crate::agents::native_runner::NativeRunner;

/// Agent runtime for executing agents
pub struct AgentRuntime {
    capability_manager: Arc<CapabilityManager>,
    wasm_host: Arc<WasmHost>,
    native_runner: Arc<NativeRunner>,
}

impl AgentRuntime {
    pub fn new() -> Result<Self> {
        let wasm_host = Arc::new(WasmHost::new()?);
        let native_runner = Arc::new(NativeRunner::new());
        let capability_manager = Arc::new(CapabilityManager::new());

        Ok(AgentRuntime {
            capability_manager,
            wasm_host,
            native_runner,
        })
    }

    /// Execute an agent
    pub async fn execute(&self, manifest: &Manifest, input: &str) -> Result<Vec<Event>> {
        // Check capabilities
        for cap_str in &manifest.capabilities {
            let cap = crate::agents::capabilities::Capability::parse(cap_str)?;
            if !self.capability_manager.check(&cap).await {
                tracing::warn!("Capability not granted: {}", cap_str);
                // In a real implementation, this would request consent
            }
        }

        // Execute based on sandbox mode
        if manifest.requires_native() {
            self.execute_native(manifest, input).await
        } else {
            self.execute_wasm(manifest, input).await
        }
    }

    async fn execute_wasm(&self, manifest: &Manifest, input: &str) -> Result<Vec<Event>> {
        tracing::info!("Executing WASM agent: {}", manifest.name);
        
        // Placeholder - real implementation would:
        // 1. Load WASM module
        // 2. Setup WASI context with capability restrictions
        // 3. Execute with input
        // 4. Stream output events
        
        Ok(vec![Event::input("wasm-agent", input.to_string(), 0)])
    }

    async fn execute_native(&self, manifest: &Manifest, input: &str) -> Result<Vec<Event>> {
        tracing::info!("Executing native agent: {}", manifest.name);
        
        // Placeholder - real implementation would:
        // 1. Spawn isolated subprocess
        // 2. Setup IPC channels
        // 3. Send input via stdin
        // 4. Stream output events from stdout
        
        Ok(vec![Event::input("native-agent", input.to_string(), 0)])
    }

    pub fn capability_manager(&self) -> Arc<CapabilityManager> {
        self.capability_manager.clone()
    }
}

impl Default for AgentRuntime {
    fn default() -> Self {
        Self::new().expect("Failed to create AgentRuntime")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "wasm")]
    fn test_runtime_creation() {
        let runtime = AgentRuntime::new();
        assert!(runtime.is_ok());
    }

    #[test]
    #[cfg(not(feature = "wasm"))]
    fn test_runtime_creation_without_wasm() {
        let runtime = AgentRuntime::new();
        // Without WASM feature, runtime creation should fail gracefully
        assert!(runtime.is_err());
    }
}
