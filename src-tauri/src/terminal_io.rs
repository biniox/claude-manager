use crate::types::OutputEvent;
use portable_pty::{Child, CommandBuilder, MasterPty, PtySize};
use std::io::Write;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Buffer size for output chunks
const BUFFER_SIZE: usize = 8192;

/// PTY process handle
#[derive(Clone)]
pub struct PtyProcess {
    pub instance_id: String,
    pub child: Arc<Mutex<Option<Box<dyn Child + Send>>>>,
    pub read_task_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl PtyProcess {
    pub fn new(
        instance_id: String,
        child: Box<dyn Child + Send>,
        read_task_handle: JoinHandle<()>,
    ) -> Self {
        Self {
            instance_id,
            child: Arc::new(Mutex::new(Some(child))),
            read_task_handle: Arc::new(Mutex::new(Some(read_task_handle))),
        }
    }

    /// Stop the process and cleanup
    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Kill the child process
        let mut child_guard = self.child.lock().await;
        if let Some(mut child) = child_guard.take() {
            child.kill()?;
        }

        // Abort the read task
        let mut task_guard = self.read_task_handle.lock().await;
        if let Some(handle) = task_guard.take() {
            handle.abort();
        }

        Ok(())
    }

    /// Check if the process is still running
    pub async fn is_running(&self) -> bool {
        self.child.lock().await.is_some()
    }
}

/// Spawn a PTY process and start reading output
pub fn spawn_pty_process(
    app_handle: AppHandle,
    instance_id: String,
    working_dir: &str,
    command: &str,
    args: &[&str],
) -> Result<PtyProcess, Box<dyn std::error::Error + Send + Sync>> {
    let pty_system = portable_pty::native_pty_system();
    let pty_pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    let mut cmd = CommandBuilder::new(command);
    for arg in args {
        cmd.arg(arg);
    }
    cmd.cwd(working_dir);

    let child = pty_pair.slave.spawn_command(cmd)?;

    let instance_id_clone = instance_id.clone();
    let mut reader = pty_pair.master.try_clone_reader()?;

    // Spawn a task to read output and send to frontend
    let read_task = tokio::spawn(async move {
        let mut buffer = [0u8; BUFFER_SIZE];

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => {
                    // EOF - process terminated
                    break;
                }
                Ok(n) => {
                    // Convert bytes to UTF-8, ignoring invalid sequences
                    let data = String::from_utf8_lossy(&buffer[..n]).to_string();

                    let event = OutputEvent {
                        instance_id: instance_id_clone.clone(),
                        data,
                    };

                    // Send to frontend via Tauri event
                    if let Err(e) = app_handle.emit("instance-output", &event) {
                        eprintln!("Failed to emit output event: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from PTY: {}", e);
                    break;
                }
            }
        }
    });

    Ok(PtyProcess::new(instance_id, child, read_task))
}

/// Write input to the PTY
pub fn write_to_pty(
    writer: &mut dyn Write,
    input: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_all(input)?;
    writer.flush()?;
    Ok(())
}

/// Resize the PTY
pub fn resize_pty(
    master: &mut dyn MasterPty,
    rows: u16,
    cols: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    master.resize(PtySize {
        rows,
        cols,
        pixel_width: 0,
        pixel_height: 0,
    })?;
    Ok(())
}
