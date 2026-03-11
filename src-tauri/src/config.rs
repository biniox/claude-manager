use crate::types::{AppConfig, Instance, InstanceError};
use dirs::config_dir;
use serde_json;
use std::fs;
use std::path::PathBuf;

/// Get the config directory path
pub fn get_config_dir() -> Result<PathBuf, InstanceError> {
    let mut path = config_dir()
        .ok_or_else(|| InstanceError::ConfigError("Could not find config directory".to_string()))?;

    path.push("claude-manager");
    Ok(path)
}

/// Get the instances config file path
pub fn get_config_file_path() -> Result<PathBuf, InstanceError> {
    let mut path = get_config_dir()?;
    path.push("instances.json");
    Ok(path)
}

/// Ensure the config directory exists
fn ensure_config_dir() -> Result<(), InstanceError> {
    let dir = get_config_dir()?;
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| {
            InstanceError::ConfigError(format!("Failed to create config directory: {}", e))
        })?;
    }
    Ok(())
}

/// Load the configuration file
pub fn load_config() -> Result<AppConfig, InstanceError> {
    ensure_config_dir()?;

    let path = get_config_file_path()?;
    if !path.exists() {
        // Create default config if file doesn't exist
        let config = AppConfig::default();
        save_config(&config)?;
        return Ok(config);
    }

    let content = fs::read_to_string(&path).map_err(|e| {
        InstanceError::ConfigError(format!("Failed to read config file: {}", e))
    })?;

    let config: AppConfig = serde_json::from_str(&content).map_err(|e| {
        InstanceError::ConfigError(format!("Failed to parse config file: {}", e))
    })?;

    Ok(config)
}

/// Save the configuration file
pub fn save_config(config: &AppConfig) -> Result<(), InstanceError> {
    ensure_config_dir()?;

    let path = get_config_file_path()?;
    let content = serde_json::to_string_pretty(config).map_err(|e| {
        InstanceError::ConfigError(format!("Failed to serialize config: {}", e))
    })?;

    fs::write(&path, content).map_err(|e| {
        InstanceError::ConfigError(format!("Failed to write config file: {}", e))
    })?;

    Ok(())
}

/// Add an instance to the configuration
pub fn add_instance(instance: Instance) -> Result<(), InstanceError> {
    let mut config = load_config()?;

    // Check if instance with same ID already exists
    if config.instances.iter().any(|i| i.id == instance.id) {
        return Err(InstanceError::ConfigError(format!(
            "Instance with ID {} already exists",
            instance.id
        )));
    }

    config.instances.push(instance);
    save_config(&config)?;
    Ok(())
}

/// Update an instance in the configuration
pub fn update_instance(instance: Instance) -> Result<(), InstanceError> {
    let mut config = load_config()?;

    let pos = config
        .instances
        .iter()
        .position(|i| i.id == instance.id)
        .ok_or_else(|| InstanceError::NotFound(instance.id.clone()))?;

    config.instances[pos] = instance;
    save_config(&config)?;
    Ok(())
}

/// Delete an instance from the configuration
pub fn delete_instance(id: &str) -> Result<(), InstanceError> {
    let mut config = load_config()?;

    let pos = config
        .instances
        .iter()
        .position(|i| i.id == id)
        .ok_or_else(|| InstanceError::NotFound(id.to_string()))?;

    config.instances.remove(pos);
    save_config(&config)?;
    Ok(())
}

/// Get an instance by ID
pub fn get_instance(id: &str) -> Result<Instance, InstanceError> {
    let config = load_config()?;
    config
        .instances
        .into_iter()
        .find(|i| i.id == id)
        .ok_or_else(|| InstanceError::NotFound(id.to_string()))
}

/// Get all instances
pub fn get_all_instances() -> Result<Vec<Instance>, InstanceError> {
    let config = load_config()?;
    Ok(config.instances)
}

/// Get all instances that should autostart
pub fn get_autostart_instances() -> Result<Vec<Instance>, InstanceError> {
    let config = load_config()?;
    Ok(config
        .instances
        .into_iter()
        .filter(|i| i.autostart)
        .collect())
}
