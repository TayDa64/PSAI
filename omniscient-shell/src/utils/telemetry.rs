#![allow(dead_code)]
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
        self.record_event(name, Some(duration_ms), metadata, success)
            .await
    }

    /// Sanitize metadata to remove sensitive information
    fn sanitize_metadata(&self, mut metadata: HashMap<String, String>) -> HashMap<String, String> {
        // Remove any keys that might contain secrets
        let sensitive_keys = ["token", "password", "secret", "key", "auth", "credential"];

        metadata.retain(|k, _| {
            !sensitive_keys
                .iter()
                .any(|&sensitive| k.to_lowercase().contains(sensitive))
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

        // Calculate average only over events that have a duration
        let events_with_duration: Vec<u64> = events.iter().filter_map(|e| e.duration_ms).collect();

        let avg_duration = if !events_with_duration.is_empty() {
            // Use u128 to prevent overflow during sum
            let sum: u128 = events_with_duration.iter().map(|&d| d as u128).sum();
            let count = events_with_duration.len() as u128;

            // Guard against divide-by-zero (already checked above, but be explicit)
            if count > 0 {
                let avg = sum / count;
                // Clamp to u64::MAX if overflow would occur
                Some(avg.min(u64::MAX as u128) as u64)
            } else {
                None
            }
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
    #[allow(dead_code)]
    pub async fn clear(&self) {
        let mut events = self.events.write().await;
        events.clear();
    }

    /// Enable telemetry
    #[allow(dead_code)]
    pub async fn enable(&self) {
        let mut config = self.config.write().await;
        config.enabled = true;
        tracing::info!("Telemetry enabled (opt-in, performance-only, no secrets)");
    }

    /// Disable telemetry
    #[allow(dead_code)]
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
        let config = TelemetryConfig {
            enabled: true,
            ..Default::default()
        };
        let collector = TelemetryCollector::new(config);

        let mut metadata = HashMap::new();
        metadata.insert("operation".to_string(), "test".to_string());

        collector
            .record_event("test_event", Some(100), metadata, true)
            .await
            .unwrap();

        let summary = collector.get_summary().await;
        assert_eq!(summary.total_events, 1);
        assert_eq!(summary.successful_events, 1);
    }

    #[tokio::test]
    async fn test_metadata_sanitization() {
        let config = TelemetryConfig {
            enabled: true,
            ..Default::default()
        };
        let collector = TelemetryCollector::new(config);

        let mut metadata = HashMap::new();
        metadata.insert("operation".to_string(), "test".to_string());
        metadata.insert("token".to_string(), "secret123".to_string()); // Should be removed

        collector
            .record_event("test", None, metadata, true)
            .await
            .unwrap();

        let events = collector.events.read().await;
        assert!(!events[0].metadata.contains_key("token"));
        assert!(events[0].metadata.contains_key("operation"));
    }

    #[tokio::test]
    async fn test_performance_metric() {
        let config = TelemetryConfig {
            enabled: true,
            ..Default::default()
        };
        let collector = TelemetryCollector::new(config);

        let metric = PerformanceMetric::new("test_operation").with_metadata("type", "unittest");

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        collector.record_performance(metric, true).await.unwrap();

        let summary = collector.get_summary().await;
        assert_eq!(summary.total_events, 1);
        assert!(summary.avg_duration_ms.unwrap() >= 10);
    }

    #[tokio::test]
    async fn test_average_with_no_duration_events() {
        let config = TelemetryConfig {
            enabled: true,
            ..Default::default()
        };
        let collector = TelemetryCollector::new(config);

        // Add events without duration
        let metadata = HashMap::new();
        collector
            .record_event("event1", None, metadata.clone(), true)
            .await
            .unwrap();
        collector
            .record_event("event2", None, metadata.clone(), true)
            .await
            .unwrap();
        collector
            .record_event("event3", None, metadata, true)
            .await
            .unwrap();

        let summary = collector.get_summary().await;
        assert_eq!(summary.total_events, 3);
        assert_eq!(summary.avg_duration_ms, None); // No duration events, so no average
    }

    #[tokio::test]
    async fn test_average_with_mixed_duration_events() {
        let config = TelemetryConfig {
            enabled: true,
            ..Default::default()
        };
        let collector = TelemetryCollector::new(config);

        let metadata = HashMap::new();
        collector
            .record_event("event1", Some(100), metadata.clone(), true)
            .await
            .unwrap();
        collector
            .record_event("event2", None, metadata.clone(), true)
            .await
            .unwrap();
        collector
            .record_event("event3", Some(200), metadata.clone(), true)
            .await
            .unwrap();
        collector
            .record_event("event4", None, metadata, true)
            .await
            .unwrap();

        let summary = collector.get_summary().await;
        assert_eq!(summary.total_events, 4);
        // Average should be (100 + 200) / 2 = 150, only counting events with duration
        assert_eq!(summary.avg_duration_ms, Some(150));
    }

    #[tokio::test]
    async fn test_average_overflow_protection() {
        let config = TelemetryConfig {
            enabled: true,
            ..Default::default()
        };
        let collector = TelemetryCollector::new(config);

        // Add events with very large durations to test overflow protection
        let metadata = HashMap::new();
        collector
            .record_event("event1", Some(u64::MAX / 2), metadata.clone(), true)
            .await
            .unwrap();
        collector
            .record_event("event2", Some(u64::MAX / 2), metadata, true)
            .await
            .unwrap();

        let summary = collector.get_summary().await;
        assert_eq!(summary.total_events, 2);
        // Should not panic and should return a valid result
        assert!(summary.avg_duration_ms.is_some());
        // The average should be around u64::MAX / 2
        assert!(summary.avg_duration_ms.unwrap() > 0);
    }

    #[tokio::test]
    async fn test_divide_by_zero_protection() {
        let collector = TelemetryCollector::default();

        // Get summary with no events
        let summary = collector.get_summary().await;
        assert_eq!(summary.total_events, 0);
        assert_eq!(summary.avg_duration_ms, None); // Should not panic
    }
}
