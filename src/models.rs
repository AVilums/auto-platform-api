use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Mirrors the `EventType` enum in the launcher's telemetry crate.
/// Keep in sync when new event types are added to the launcher.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum EventType {
    LauncherStartup,
    DownloadStart,
    DownloadComplete,
    DownloadFailure,
    ExecutionStart,
    ExecutionSuccess,
    ExecutionFailure,
    /// Catch-all so unknown variants don't break deserialisation.
    #[serde(other)]
    Unknown,
}

/// A single telemetry event, matching the launcher's `TelemetryEvent` struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub tool_name: Option<String>,
    pub tool_version: Option<String>,
    pub status: Option<String>,
    pub details: Option<String>,
}

/// The batch payload sent by the launcher in a single HTTP request.
#[derive(Debug, Deserialize)]
pub struct EventBatch {
    pub events: Vec<TelemetryEvent>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserialises_known_event_type() {
        let raw = json!({
            "id": "abc123",
            "timestamp": "2024-01-01T00:00:00Z",
            "event_type": "ExecutionSuccess",
            "tool_name": "data-processor",
            "tool_version": "1.2.0",
            "status": "ok",
            "details": null
        });
        let event: TelemetryEvent = serde_json::from_value(raw).unwrap();
        assert_eq!(event.event_type, EventType::ExecutionSuccess);
        assert_eq!(event.tool_name.as_deref(), Some("data-processor"));
    }

    #[test]
    fn deserialises_unknown_event_type_gracefully() {
        let raw = json!({
            "id": "xyz",
            "timestamp": "2024-06-01T12:00:00Z",
            "event_type": "SomeFutureEvent",
            "tool_name": null,
            "tool_version": null,
            "status": null,
            "details": null
        });
        let event: TelemetryEvent = serde_json::from_value(raw).unwrap();
        assert_eq!(event.event_type, EventType::Unknown);
    }

    #[test]
    fn deserialises_batch() {
        let raw = json!({
            "events": [
                {
                    "id": "1",
                    "timestamp": "2024-01-01T00:00:00Z",
                    "event_type": "LauncherStartup",
                    "tool_name": null,
                    "tool_version": null,
                    "status": null,
                    "details": null
                }
            ]
        });
        let batch: EventBatch = serde_json::from_value(raw).unwrap();
        assert_eq!(batch.events.len(), 1);
    }
}
