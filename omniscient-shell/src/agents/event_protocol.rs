//! Event protocol v0.1 for agent communication

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Event protocol version
pub const PROTOCOL_VERSION: &str = "0.1";

/// Base event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_type: EventType,
    pub agent_id: String,
    pub timestamp: SystemTime,
    pub sequence: u64,
}

/// Event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EventType {
    Input(InputEvent),
    Output(OutputEvent),
    Artifact(ArtifactEvent),
    ConsentRequest(ConsentRequestEvent),
    ConsentGrant(ConsentGrantEvent),
    ConsentRevoke(ConsentRevokeEvent),
    Error(ErrorEvent),
    StateUpdate(StateUpdateEvent),
}

/// Input event: user or system input to agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputEvent {
    pub prompt: String,
    pub context_refs: Vec<String>,
}

/// Output event: agent response chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputEvent {
    pub chunk_id: u64,
    pub content_type: String, // "text/plain", "text/markdown", "application/json"
    pub data: Vec<u8>,
    pub complete: bool,
}

/// Artifact event: generated artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactEvent {
    pub id: String,
    pub kind: String, // "diff", "log", "preview", "code"
    pub path: String,
    pub preview_hint: Option<String>,
}

/// Consent request event: agent requests capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRequestEvent {
    pub capability: String,
    pub reason: String,
    pub duration_s: Option<u64>,
}

/// Consent grant event: user grants capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentGrantEvent {
    pub capability: String,
    pub expires_at: Option<SystemTime>,
}

/// Consent revoke event: capability revoked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRevokeEvent {
    pub capability: String,
}

/// Error event: agent error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEvent {
    pub code: String,
    pub message: String,
    pub hint: Option<String>,
}

/// State update event: agent state change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateUpdateEvent {
    pub key: String,
    pub value: serde_json::Value,
    pub scope: String, // "agent", "session", "global"
}

impl Event {
    pub fn new(event_type: EventType, agent_id: impl Into<String>, sequence: u64) -> Self {
        Event {
            event_type,
            agent_id: agent_id.into(),
            timestamp: SystemTime::now(),
            sequence,
        }
    }

    pub fn input(agent_id: impl Into<String>, prompt: String, sequence: u64) -> Self {
        Event::new(
            EventType::Input(InputEvent {
                prompt,
                context_refs: vec![],
            }),
            agent_id,
            sequence,
        )
    }

    pub fn output(
        agent_id: impl Into<String>,
        chunk_id: u64,
        content_type: impl Into<String>,
        data: Vec<u8>,
        complete: bool,
        sequence: u64,
    ) -> Self {
        Event::new(
            EventType::Output(OutputEvent {
                chunk_id,
                content_type: content_type.into(),
                data,
                complete,
            }),
            agent_id,
            sequence,
        )
    }

    pub fn error(
        agent_id: impl Into<String>,
        code: impl Into<String>,
        message: impl Into<String>,
        sequence: u64,
    ) -> Self {
        Event::new(
            EventType::Error(ErrorEvent {
                code: code.into(),
                message: message.into(),
                hint: None,
            }),
            agent_id,
            sequence,
        )
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_serialization() {
        let event = Event::input("test-agent", "Hello".to_string(), 1);
        let json = event.to_json().unwrap();
        let parsed = Event::from_json(&json).unwrap();

        assert_eq!(event.agent_id, parsed.agent_id);
        assert_eq!(event.sequence, parsed.sequence);
    }

    #[test]
    fn test_output_event() {
        let event = Event::output(
            "test-agent",
            1,
            "text/plain",
            b"Hello, world!".to_vec(),
            true,
            2,
        );

        match event.event_type {
            EventType::Output(output) => {
                assert_eq!(output.chunk_id, 1);
                assert_eq!(output.content_type, "text/plain");
                assert!(output.complete);
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_error_event() {
        let event = Event::error("test-agent", "ERR001", "Something failed", 3);

        match event.event_type {
            EventType::Error(error) => {
                assert_eq!(error.code, "ERR001");
                assert_eq!(error.message, "Something failed");
            }
            _ => panic!("Wrong event type"),
        }
    }
}
