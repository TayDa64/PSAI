//! OAuth shim when omniscience feature is disabled
//! Provides minimal stubs to allow compilation without full OAuth functionality

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Minimal shim for OAuthBroker
pub struct OAuthBroker;

impl OAuthBroker {
    pub fn new_disabled() -> Self {
        OAuthBroker
    }
}

/// Minimal shim for TokenVault
pub struct TokenVault;

impl TokenVault {
    pub fn new_in_memory() -> Self {
        tracing::warn!("OAuth functionality is disabled. To enable, build with --features omniscience");
        TokenVault
    }
}

/// Minimal shim for ConsentLedger
pub struct ConsentLedger;

impl ConsentLedger {
    pub fn new_disabled() -> Result<Self> {
        tracing::info!("ConsentLedger disabled (omniscience feature not enabled)");
        Ok(ConsentLedger)
    }
}

/// Provider configuration stub
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub client_id: String,
    pub auth_url: String,
    pub token_url: String,
    pub device_auth_url: Option<String>,
    pub scopes: Vec<String>,
}

/// Token handle stub
#[derive(Debug, Clone)]
pub struct TokenHandle {
    pub id: String,
    pub provider: String,
    pub scopes: Vec<String>,
}

/// Event stub for when agents module is not available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub timestamp: SystemTime,
    pub event_type: EventType,
    pub agent_id: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Input,
    Output,
    Error,
}

impl Event {
    pub fn input(agent_id: &str, data: String, _sequence: u64) -> Self {
        Event {
            timestamp: SystemTime::now(),
            event_type: EventType::Input,
            agent_id: agent_id.to_string(),
            data,
        }
    }
}

