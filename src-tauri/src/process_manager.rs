use crate::types::{Instance, InstanceError, InstanceStatus};
use portable_pty::{Child, CommandBuilder, MasterPty, NativePtySystem, PtySize, PtySystem};
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;

/// Active Claude process with PTY
pub struct ClaudeProcess {
    pub instance_id: String,
    pub master_writer: Box<dyn Write + Send>,
    pub master_resize: Box<dyn MasterPty + Send>,
    pub child: Box<dyn Child + Send>,
    pub status: InstanceStatus,
}

impl ClaudeProcess {
    pub fn new(
        instance_id: String,
        master_writer: Box<dyn Write + Send>,
        master_resize: Box<dyn MasterPty + Send>,
        child: Box<dyn Child + Send>,
    ) -> Self {
        Self {
            instance_id,
            master_writer,
            master_resize,
            child,
            status: InstanceStatus::Running,
        }
    }

    pub fn write(&mut self, input: &str) -> Result<(), std::io::Error> {
        self.master_writer.write_all(input.as_bytes())?;
        self.master_writer.flush()?;
        Ok(())
    }

    pub fn resize(&mut self, rows: u16, cols: u16) -> Result<(), Box<dyn std::error::Error>> {
        self.master_resize.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        Ok(())
    }
}

/// Manager for all running Claude instances
pub struct ProcessManager {
    processes: Arc<Mutex<HashMap<String, ClaudeProcess>>>,
    app_handle: AppHandle,
}

impl ProcessManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            processes: Arc::new(Mutex::new(HashMap::new())),
            app_handle,
        }
    }

    /// Start a Claude instance in the given folder
    pub async fn start_instance(
        &self,
        instance: &Instance,
    ) -> Result<(), InstanceError> {
        let processes = self.processes.clone();
        let instance_id = instance.id.clone();

        // Check if already running
        {
            let processes_guard = processes.lock().await;
            if processes_guard.contains_key(&instance_id) {
                return Err(InstanceError::AlreadyRunning(instance_id));
            }
        }

        // Create PTY
        let pty_system = NativePtySystem::default();
        let pty_pair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| InstanceError::ProcessError(format!("Failed to open PTY: {}", e)))?;

        // Create command for "claude"
        let mut cmd = CommandBuilder::new("claude");
        cmd.cwd(&instance.folder_path);

        // Spawn the child process
        let child = pty_pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| InstanceError::ProcessError(format!("Failed to spawn process: {}", e)))?;

        // Clone master for reading and writing
        let mut reader = pty_pair
            .master
            .try_clone_reader()
            .map_err(|e| InstanceError::ProcessError(format!("Failed to clone reader: {}", e)))?;
        let writer = pty_pair
            .master
            .take_writer()
            .map_err(|e| InstanceError::ProcessError(format!("Failed to get writer: {}", e)))?;
        let resizer = pty_pair.master;

        let instance_id_for_task = instance_id.clone();
        let instance_id_for_spawn = instance_id.clone();
        let app_handle_clone = self.app_handle.clone();
        let processes_for_spawn = processes.clone();

        // Spawn task to read output and send to frontend
        tokio::spawn(async move {
            const BUFFER_SIZE: usize = 8192;
            let mut buffer = [0u8; BUFFER_SIZE];

            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => {
                        // EOF - process terminated
                        if let Ok(mut processes) = processes_for_spawn.try_lock() {
                            if let Some(proc) = processes.get_mut(&instance_id_for_task) {
                                proc.status = InstanceStatus::Stopped;
                            }
                        }

                        // Send termination event
                        let _ = app_handle_clone.emit(
                            "instance-terminated",
                            crate::types::OutputEvent {
                                instance_id: instance_id_for_task.clone(),
                                data: "\r\n[Process terminated]".to_string(),
                            },
                        );
                        break;
                    }
                    Ok(n) => {
                        let data = String::from_utf8_lossy(&buffer[..n]).to_string();
                        let event = crate::types::OutputEvent {
                            instance_id: instance_id_for_task.clone(),
                            data,
                        };
                        let _ = app_handle_clone.emit("instance-output", &event);
                    }
                    Err(e) => {
                        eprintln!("Error reading from PTY: {}", e);

                        if let Ok(mut processes) = processes_for_spawn.try_lock() {
                            if let Some(proc) = processes.get_mut(&instance_id_for_task) {
                                proc.status = InstanceStatus::Error;
                            }
                        }
                        break;
                    }
                }
            }
        });

        // Store the process
        {
            let mut processes_guard = processes.lock().await;
            processes_guard.insert(
                instance_id,
                ClaudeProcess::new(instance_id_for_spawn, writer, resizer, child),
            );
        }

        Ok(())
    }

    /// Stop a running Claude instance
    pub async fn stop_instance(&self, instance_id: &str) -> Result<(), InstanceError> {
        let mut processes_guard = self.processes.lock().await;
        let mut process = processes_guard
            .remove(instance_id)
            .ok_or_else(|| InstanceError::NotFound(instance_id.to_string()))?;

        // Kill the child process
        process
            .child
            .kill()
            .map_err(|e| InstanceError::ProcessError(format!("Failed to kill process: {}", e)))?;

        Ok(())
    }

    /// Restart a Claude instance
    pub async fn restart_instance(
        &self,
        instance: &Instance,
    ) -> Result<(), InstanceError> {
        // First stop if running
        if self.is_running(&instance.id).await {
            self.stop_instance(&instance.id).await?;
        }

        // Then start again
        self.start_instance(instance).await
    }

    /// Write input to a Claude instance's PTY
    pub async fn write_to_instance(
        &self,
        instance_id: &str,
        input: &str,
    ) -> Result<(), InstanceError> {
        let mut processes_guard = self.processes.lock().await;
        let process = processes_guard
            .get_mut(instance_id)
            .ok_or_else(|| InstanceError::NotFound(instance_id.to_string()))?;

        process
            .write(input)
            .map_err(|e| InstanceError::ProcessError(format!("Failed to write to PTY: {}", e)))?;

        Ok(())
    }

    /// Resize a Claude instance's PTY
    pub async fn resize_instance(
        &self,
        instance_id: &str,
        rows: u16,
        cols: u16,
    ) -> Result<(), InstanceError> {
        let mut processes_guard = self.processes.lock().await;
        let process = processes_guard
            .get_mut(instance_id)
            .ok_or_else(|| InstanceError::NotFound(instance_id.to_string()))?;

        process
            .resize(rows, cols)
            .map_err(|e| InstanceError::ProcessError(format!("Failed to resize PTY: {}", e)))?;

        Ok(())
    }

    /// Get status of an instance
    pub async fn get_instance_status(&self, instance_id: &str) -> InstanceStatus {
        let processes_guard = self.processes.lock().await;
        match processes_guard.get(instance_id) {
            Some(proc) => proc.status.clone(),
            None => InstanceStatus::Stopped,
        }
    }

    /// Check if an instance is running
    pub async fn is_running(&self, instance_id: &str) -> bool {
        let processes_guard = self.processes.lock().await;
        processes_guard.contains_key(instance_id)
    }

    /// Get all running instance IDs
    pub async fn get_running_instances(&self) -> Vec<String> {
        let processes_guard = self.processes.lock().await;
        processes_guard.keys().cloned().collect()
    }

    /// Stop all running instances
    pub async fn stop_all(&self) {
        let instance_ids = self.get_running_instances().await;
        for instance_id in instance_ids {
            let _ = self.stop_instance(&instance_id).await;
        }
    }
}

/// Global process manager reference (set in main)
#[allow(static_mut_refs)]
static mut PROCESS_MANAGER: Option<ProcessManager> = None;

/// Initialize the global process manager
pub fn init_process_manager(app_handle: AppHandle) {
    unsafe {
        PROCESS_MANAGER = Some(ProcessManager::new(app_handle));
    }
}

/// Get a reference to the global process manager
pub fn get_process_manager() -> &'static ProcessManager {
    unsafe {
        PROCESS_MANAGER
            .as_ref()
            .expect("Process manager not initialized")
    }
}
