use serde::{Deserialize, Serialize};

/// Instance status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InstanceStatus {
    Stopped,
    Running,
    Error,
}

/// Instance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub folder_path: String,
    pub autostart: bool,
    pub created_at: String, // ISO 8601 timestamp
}

/// Instance status with output buffer (for frontend queries)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceStatusInfo {
    pub id: String,
    pub status: InstanceStatus,
    pub output_buffer: Option<String>, // Recent output (limited size)
}

/// Configuration file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub instances: Vec<Instance>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            instances: Vec::new(),
        }
    }
}

/// Output event payload for Tauri events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputEvent {
    pub instance_id: String,
    pub data: String,
}

/// Error types for instance operations
#[derive(Debug, thiserror::Error)]
pub enum InstanceError {
    #[error("Instance not found: {0}")]
    NotFound(String),

    #[error("Instance already running: {0}")]
    AlreadyRunning(String),

    #[error("Process error: {0}")]
    ProcessError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Config error: {0}")]
    ConfigError(String),
}

impl From<InstanceError> for String {
    fn from(err: InstanceError) -> Self {
        err.to_string()
    }
}
