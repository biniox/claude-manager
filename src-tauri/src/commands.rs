use crate::config::{
    add_instance, delete_instance, get_all_instances, get_autostart_instances, get_instance,
    update_instance,
};
use crate::process_manager::get_process_manager;
use crate::types::{Instance, InstanceError, InstanceStatusInfo};
use chrono::Utc;
use std::path::Path;
use uuid::Uuid;

/// Create a new instance
#[tauri::command]
pub async fn create_instance(
    name: String,
    folder_path: String,
    autostart: bool,
) -> Result<Instance, String> {
    // Validate folder path exists
    if !Path::new(&folder_path).exists() {
        return Err(InstanceError::ConfigError(format!(
            "Folder path does not exist: {}",
            folder_path
        ))
        .to_string());
    }

    let instance = Instance {
        id: Uuid::new_v4().to_string(),
        name,
        folder_path,
        autostart,
        created_at: Utc::now().to_rfc3339(),
    };

    add_instance(instance.clone()).map_err(|e| e.to_string())?;

    Ok(instance)
}

/// Get all instances
#[tauri::command]
pub async fn get_instances() -> Result<Vec<Instance>, String> {
    get_all_instances().map_err(|e| e.to_string())
}

/// Delete an instance
#[tauri::command]
pub async fn delete_instance_command(id: String) -> Result<(), String> {
    // Stop the instance if running
    let process_manager = get_process_manager();
    if process_manager.is_running(&id).await {
        process_manager.stop_instance(&id).await.map_err(|e| e.to_string())?;
    }

    delete_instance(&id).map_err(|e| e.to_string())?;

    Ok(())
}

/// Update an instance
#[tauri::command]
pub async fn update_instance_command(
    id: String,
    name: Option<String>,
    folder_path: Option<String>,
    autostart: Option<bool>,
) -> Result<Instance, String> {
    let mut instance = get_instance(&id).map_err(|e| e.to_string())?;

    if let Some(name) = name {
        instance.name = name;
    }
    if let Some(folder_path) = folder_path {
        if !Path::new(&folder_path).exists() {
            return Err(InstanceError::ConfigError(format!(
                "Folder path does not exist: {}",
                folder_path
            ))
            .to_string());
        }
        instance.folder_path = folder_path;
    }
    if let Some(autostart) = autostart {
        instance.autostart = autostart;
    }

    update_instance(instance.clone()).map_err(|e| e.to_string())?;

    Ok(instance)
}

/// Start an instance
#[tauri::command]
pub async fn start_instance(id: String) -> Result<(), String> {
    let instance = get_instance(&id).map_err(|e| e.to_string())?;
    let process_manager = get_process_manager();
    process_manager.start_instance(&instance).await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Stop an instance
#[tauri::command]
pub async fn stop_instance(id: String) -> Result<(), String> {
    let process_manager = get_process_manager();
    process_manager.stop_instance(&id).await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Restart an instance
#[tauri::command]
pub async fn restart_instance(id: String) -> Result<(), String> {
    let instance = get_instance(&id).map_err(|e| e.to_string())?;
    let process_manager = get_process_manager();
    process_manager.restart_instance(&instance).await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Write input to an instance's terminal
#[tauri::command]
pub async fn write_to_terminal(id: String, input: String) -> Result<(), String> {
    let process_manager = get_process_manager();
    process_manager
        .write_to_instance(&id, &input)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Get instance status
#[tauri::command]
pub async fn get_instance_status(id: String) -> Result<InstanceStatusInfo, String> {
    let process_manager = get_process_manager();
    let status = process_manager.get_instance_status(&id).await;

    Ok(InstanceStatusInfo {
        id,
        status,
        output_buffer: None,
    })
}

/// Resize instance terminal
#[tauri::command]
pub async fn resize_terminal(id: String, rows: u16, cols: u16) -> Result<(), String> {
    let process_manager = get_process_manager();
    process_manager
        .resize_instance(&id, rows, cols)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Get all autostart instances
#[tauri::command]
pub async fn get_autostart_instances_command() -> Result<Vec<Instance>, String> {
    get_autostart_instances().map_err(|e| e.to_string())
}

/// Start all autostart instances (called on app launch)
#[tauri::command]
pub async fn start_autostart_instances() -> Result<(), String> {
    let instances = get_autostart_instances().map_err(|e| e.to_string())?;
    let process_manager = get_process_manager();

    for instance in instances {
        if let Err(e) = process_manager.start_instance(&instance).await {
            eprintln!("Failed to start autostart instance {}: {}", instance.id, e);
        }
    }

    Ok(())
}
