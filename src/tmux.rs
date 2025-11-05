//! TMUX utility functions for managing sessions and windows.

use anyhow::{Result, anyhow};
use std::process::{Command, Output};

/// Check if tmux is installed and available.
pub fn check_tmux_available() -> Result<()> {
    let output = Command::new("tmux")
        .arg("-V")
        .output();

    match output {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow!("tmux is not installed or not available in PATH")),
    }
}

/// Check if a session with the given name exists.
pub fn has_session(name: &str) -> Result<bool> {
    let output = Command::new("tmux")
        .arg("has-session")
        .arg("-t")
        .arg(name)
        .output()?;

    Ok(output.status.success())
}

/// List all windows in a session.
/// Returns a vector of window names (or indices if no name).
pub fn list_windows(session: &str) -> Result<Vec<String>> {
    let output = Command::new("tmux")
        .arg("list-windows")
        .arg("-t")
        .arg(session)
        .arg("-F")
        .arg("#{window_name}")
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to list windows for session '{}'", session));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let windows = stdout
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(windows)
}

/// Create a new tmux session.
pub fn new_session(name: &str, detached: bool) -> Result<()> {
    let mut cmd = Command::new("tmux");
    cmd.arg("new-session");

    if detached {
        cmd.arg("-d");
    }

    cmd.arg("-s").arg(name);

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to create session '{}': {}", name, stderr));
    }

    Ok(())
}

/// Create a new window in an existing session.
pub fn new_window(session: &str, window_name: Option<&str>, target_index: Option<usize>) -> Result<()> {
    let mut cmd = Command::new("tmux");
    cmd.arg("new-window");
    cmd.arg("-t").arg(session);

    if let Some(idx) = target_index {
        cmd.arg("-t").arg(format!("{}:{}", session, idx));
    }

    if let Some(name) = window_name {
        cmd.arg("-n").arg(name);
    }

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to create window in session '{}': {}", session, stderr));
    }

    Ok(())
}

/// Send keys/commands to a tmux window.
/// This executes a command in the specified window.
pub fn send_keys(session: &str, window_index: usize, command: &[String]) -> Result<()> {
    let target = format!("{}:{}", session, window_index);
    let cmd_str = command.join(" ");

    let output = Command::new("tmux")
        .arg("send-keys")
        .arg("-t")
        .arg(&target)
        .arg(&cmd_str)
        .arg("C-m") // Enter key
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to send keys to '{}': {}", target, stderr));
    }

    Ok(())
}

/// Kill a tmux session.
pub fn kill_session(name: &str) -> Result<()> {
    let output = Command::new("tmux")
        .arg("kill-session")
        .arg("-t")
        .arg(name)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to kill session '{}': {}", name, stderr));
    }

    Ok(())
}

/// Kill a specific window in a session.
pub fn kill_window(session: &str, window_name: &str) -> Result<()> {
    let target = format!("{}:{}", session, window_name);

    let output = Command::new("tmux")
        .arg("kill-window")
        .arg("-t")
        .arg(&target)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to kill window '{}': {}", target, stderr));
    }

    Ok(())
}

/// Attach to a tmux session (foreground operation).
/// This will block until the user detaches.
pub fn attach_session(name: &str) -> Result<()> {
    let status = Command::new("tmux")
        .arg("attach-session")
        .arg("-t")
        .arg(name)
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to attach to session '{}'", name));
    }

    Ok(())
}
