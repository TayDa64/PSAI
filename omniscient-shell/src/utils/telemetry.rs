//! Telemetry system (opt-in only, performance metrics, no secrets)

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;

/// Telemetry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub endpoint: Option<String>,
    pub sample_rate: f32, // 0.0 to 1.0
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        TelemetryConfig {
            enabled: false, // Opt-in only
            endpoint: None,
            sample_rate: 1.0,
        }
    }
}

/// Telemetry event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub timestamp: SystemTime,
    pub event_type: String,
    pub duration_ms: Option<u64>,
    pub metadata: HashMap<String, String>,
    pub success: bool,
}

/// Performance metric
#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    pub name: String,
    pub start: Instant,
    pub metadata: HashMap<String, String>,
}

impl PerformanceMetric {
    pub fn new(name: impl Into<String>) -> Self {
        PerformanceMetric {
            name: name.into(),
            start: Instant::now(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn finish(self) -> Duration {
        self.start.elapsed()
    }
}

/// Telemetry collector
pub struct TelemetryCollector {
    config: Arc<RwLock<TelemetryConfig>>,
    events: Arc<RwLock<Vec<TelemetryEvent>>>,
}

impl TelemetryCollector {
    pub fn new(config: TelemetryConfig) -> Self {
        TelemetryCollector {
            config: Arc::new(RwLock::new(config)),
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Check if telemetry is enabled
    pub async fn is_enabled(&self) -> bool {
        let config = self.config.read().await;
        config.enabled
    }

    /// Record a telemetry event
    pub async fn record_event(
        &self,
        event_type: impl Into<String>,
        duration_ms: Option<u64>,
        metadata: HashMap<String, String>,
        success: bool,
    ) -> Result<()> {
        if !self.is_enabled().await {
            return Ok(());
        }

        // Sample based on rate
        let config = self.config.read().await;
        if rand::random::<f32>() > config.sample_rate {
            return Ok(());
        }

        let event = TelemetryEvent {
            timestamp: SystemTime::now(),
            event_type: event_type.into(),
            duration_ms,
            metadata: self.sanitize_metadata(metadata),
            success,
        };

        let mut events = self.events.write().await;
        events.push(event);

        // Limit buffer size
        if events.len() > 1000 {
            events.drain(0..500); // Keep most recent 500
        }

        Ok(())
    }

    /// Record a performance metric
    pub async fn record_performance(&self, metric: PerformanceMetric, success: bool) -> Result<()> {
        let name = metric.name.clone();
        let metadata = metric.metadata.clone();
        let duration_ms = metric.finish().as_millis() as u64;
        self.record_event(name, Some(duration_ms), metadata, success).await
    }

    /// Sanitize metadata to remove sensitive information
    fn sanitize_metadata(&self, mut metadata: HashMap<String, String>) -> HashMap<String, String> {
        // Remove any keys that might contain secrets
        let sensitive_keys = ["token", "password", "secret", "key", "auth", "credential"];
        
        metadata.retain(|k, _| {
            !sensitive_keys.iter().any(|&sensitive| k.to_lowercase().contains(sensitive))
        });

        // Truncate long values
        for value in metadata.values_mut() {
            if value.len() > 100 {
                value.truncate(97);
                value.push_str("...");
            }
        }

        metadata
    }

    /// Get summary statistics
    pub async fn get_summary(&self) -> TelemetrySummary {
        let events = self.events.read().await;
        
        let total_events = events.len();
        let successful_events = events.iter().filter(|e| e.success).count();
        let failed_events = total_events - successful_events;

        let avg_duration = if !events.is_empty() {
            let sum: u64 = events.iter()
                .filter_map(|e| e.duration_ms)
                .sum();
            Some(sum / events.len() as u64)
        } else {
            None
        };

        TelemetrySummary {
            total_events,
            successful_events,
            failed_events,
            avg_duration_ms: avg_duration,
        }
    }

    /// Clear all events
    pub async fn clear(&self) {
        let mut events = self.events.write().await;
        events.clear();
    }

    /// Enable telemetry
    pub async fn enable(&self) {
        let mut config = self.config.write().await;
        config.enabled = true;
        tracing::info!("Telemetry enabled (opt-in, performance-only, no secrets)");
    }

    /// Disable telemetry
    pub async fn disable(&self) {
        let mut config = self.config.write().await;
        config.enabled = false;
        
        // Clear existing data
        let mut events = self.events.write().await;
        events.clear();
        
        tracing::info!("Telemetry disabled and data cleared");
    }
}

impl Default for TelemetryCollector {
    fn default() -> Self {
        Self::new(TelemetryConfig::default())
    }
}

/// Telemetry summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySummary {
    pub total_events: usize,
    pub successful_events: usize,
    pub failed_events: usize,
    pub avg_duration_ms: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_telemetry_opt_in() {
        let collector = TelemetryCollector::default();
        assert!(!collector.is_enabled().await);
    }

    #[tokio::test]
    async fn test_record_event() {
        let mut config = TelemetryConfig::default();
        config.enabled = true;
        let collector = TelemetryCollector::new(config);

        let mut metadata = HashMap::new();
        metadata.insert("operation".to_string(), "test".to_string());

        collector.record_event("test_event", Some(100), metadata, true).await.unwrap();

        let summary = collector.get_summary().await;
        assert_eq!(summary.total_events, 1);
        assert_eq!(summary.successful_events, 1);
    }

    #[tokio::test]
    async fn test_metadata_sanitization() {
        let mut config = TelemetryConfig::default();
        config.enabled = true;
        let collector = TelemetryCollector::new(config);

        let mut metadata = HashMap::new();
        metadata.insert("operation".to_string(), "test".to_string());
        metadata.insert("token".to_string(), "secret123".to_string()); // Should be removed

        collector.record_event("test", None, metadata, true).await.unwrap();

        let events = collector.events.read().await;
        assert!(!events[0].metadata.contains_key("token"));
        assert!(events[0].metadata.contains_key("operation"));
    }

    #[tokio::test]
    async fn test_performance_metric() {
        let mut config = TelemetryConfig::default();
        config.enabled = true;
        let collector = TelemetryCollector::new(config);

        let metric = PerformanceMetric::new("test_operation")
            .with_metadata("type", "unittest");

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        collector.record_performance(metric, true).await.unwrap();

        let summary = collector.get_summary().await;
        assert_eq!(summary.total_events, 1);
        assert!(summary.avg_duration_ms.unwrap() >= 10);
    }
}
