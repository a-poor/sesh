//! TMUX utility functions for managing sessions and windows.

use anyhow::{Result, anyhow};
use std::process::Command;

/// Trait for tmux backend operations, allowing for testing with mock implementations.
pub trait TmuxBackend {
    /// Check if tmux is installed and available.
    fn check_available(&self) -> Result<()>;

    /// Check if a session with the given name exists.
    fn has_session(&self, name: &str) -> Result<bool>;

    /// List all windows in a session.
    fn list_windows(&self, session: &str) -> Result<Vec<String>>;

    /// Create a new tmux session.
    fn new_session(&self, name: &str, detached: bool) -> Result<()>;

    /// Create a new window in an existing session.
    fn new_window(
        &self,
        session: &str,
        window_name: Option<&str>,
        target_index: Option<usize>,
    ) -> Result<()>;

    /// Send keys/commands to a tmux window.
    fn send_keys(&self, session: &str, window_index: usize, command: &[String]) -> Result<()>;

    /// Kill a tmux session.
    fn kill_session(&self, name: &str) -> Result<()>;

    /// Kill a specific window in a session.
    fn kill_window(&self, session: &str, window_name: &str) -> Result<()>;

    /// Rename a window in a session.
    fn rename_window(&self, session: &str, window_index: usize, new_name: &str) -> Result<()>;

    /// Attach to a tmux session (foreground operation).
    fn attach_session(&self, name: &str) -> Result<()>;
}

/// Real tmux backend that executes actual tmux commands.
pub struct RealTmuxBackend;

impl TmuxBackend for RealTmuxBackend {
    fn check_available(&self) -> Result<()> {
        let output = Command::new("tmux").arg("-V").output();

        match output {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow!("tmux is not installed or not available in PATH")),
        }
    }

    fn has_session(&self, name: &str) -> Result<bool> {
        let output = Command::new("tmux")
            .arg("has-session")
            .arg("-t")
            .arg(name)
            .output()?;

        Ok(output.status.success())
    }

    fn list_windows(&self, session: &str) -> Result<Vec<String>> {
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

    fn new_session(&self, name: &str, detached: bool) -> Result<()> {
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

    fn new_window(
        &self,
        session: &str,
        window_name: Option<&str>,
        target_index: Option<usize>,
    ) -> Result<()> {
        let mut cmd = Command::new("tmux");
        cmd.arg("new-window");

        // Build the target based on whether we have an index
        let target = if let Some(idx) = target_index {
            format!("{}:{}", session, idx)
        } else {
            session.to_string()
        };
        cmd.arg("-t").arg(target);

        if let Some(name) = window_name {
            cmd.arg("-n").arg(name);
        }

        let output = cmd.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!(
                "Failed to create window in session '{}': {}",
                session,
                stderr
            ));
        }

        Ok(())
    }

    fn send_keys(&self, session: &str, window_index: usize, command: &[String]) -> Result<()> {
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

    fn kill_session(&self, name: &str) -> Result<()> {
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

    fn kill_window(&self, session: &str, window_name: &str) -> Result<()> {
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

    fn rename_window(&self, session: &str, window_index: usize, new_name: &str) -> Result<()> {
        let target = format!("{}:{}", session, window_index);

        let output = Command::new("tmux")
            .arg("rename-window")
            .arg("-t")
            .arg(&target)
            .arg(new_name)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Failed to rename window '{}': {}", target, stderr));
        }

        Ok(())
    }

    fn attach_session(&self, name: &str) -> Result<()> {
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
}

// Convenience functions using the real backend for backward compatibility
static REAL_BACKEND: RealTmuxBackend = RealTmuxBackend;

/// Check if tmux is installed and available.
pub fn check_tmux_available() -> Result<()> {
    REAL_BACKEND.check_available()
}

/// Check if a session with the given name exists.
pub fn has_session(name: &str) -> Result<bool> {
    REAL_BACKEND.has_session(name)
}

/// List all windows in a session.
pub fn list_windows(session: &str) -> Result<Vec<String>> {
    REAL_BACKEND.list_windows(session)
}

/// Create a new tmux session.
pub fn new_session(name: &str, detached: bool) -> Result<()> {
    REAL_BACKEND.new_session(name, detached)
}

/// Create a new window in an existing session.
pub fn new_window(
    session: &str,
    window_name: Option<&str>,
    target_index: Option<usize>,
) -> Result<()> {
    REAL_BACKEND.new_window(session, window_name, target_index)
}

/// Send keys/commands to a tmux window.
pub fn send_keys(session: &str, window_index: usize, command: &[String]) -> Result<()> {
    REAL_BACKEND.send_keys(session, window_index, command)
}

/// Kill a tmux session.
pub fn kill_session(name: &str) -> Result<()> {
    REAL_BACKEND.kill_session(name)
}

/// Kill a specific window in a session.
pub fn kill_window(session: &str, window_name: &str) -> Result<()> {
    REAL_BACKEND.kill_window(session, window_name)
}

/// Rename a window in a session.
pub fn rename_window(session: &str, window_index: usize, new_name: &str) -> Result<()> {
    REAL_BACKEND.rename_window(session, window_index, new_name)
}

/// Attach to a tmux session (foreground operation).
pub fn attach_session(name: &str) -> Result<()> {
    REAL_BACKEND.attach_session(name)
}

#[cfg(test)]
use std::sync::{Arc, Mutex};

#[cfg(test)]
use std::collections::HashMap;

/// Mock tmux backend for testing.
#[cfg(test)]
#[derive(Clone)]
pub struct MockTmuxBackend {
    state: Arc<Mutex<MockState>>,
}

#[cfg(test)]
#[derive(Default)]
struct MockState {
    sessions: HashMap<String, Vec<String>>, // session_name -> window_names
    commands_sent: Vec<(String, usize, Vec<String>)>, // (session, window_idx, command)
}

#[cfg(test)]
impl MockTmuxBackend {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(MockState::default())),
        }
    }

    pub fn with_session(self, name: &str, windows: Vec<&str>) -> Self {
        let mut state = self.state.lock().unwrap();
        state.sessions.insert(
            name.to_string(),
            windows.iter().map(|w| w.to_string()).collect(),
        );
        drop(state);
        self
    }

    pub fn get_sessions(&self) -> HashMap<String, Vec<String>> {
        self.state.lock().unwrap().sessions.clone()
    }

    pub fn get_commands_sent(&self) -> Vec<(String, usize, Vec<String>)> {
        self.state.lock().unwrap().commands_sent.clone()
    }
}

#[cfg(test)]
impl TmuxBackend for MockTmuxBackend {
    fn check_available(&self) -> Result<()> {
        Ok(())
    }

    fn has_session(&self, name: &str) -> Result<bool> {
        let state = self.state.lock().unwrap();
        Ok(state.sessions.contains_key(name))
    }

    fn list_windows(&self, session: &str) -> Result<Vec<String>> {
        let state = self.state.lock().unwrap();
        state
            .sessions
            .get(session)
            .cloned()
            .ok_or_else(|| anyhow!("Session '{}' not found", session))
    }

    fn new_session(&self, name: &str, _detached: bool) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        if state.sessions.contains_key(name) {
            return Err(anyhow!("Session '{}' already exists", name));
        }
        // Create session with default window at index 0 (matches real tmux behavior)
        state.sessions.insert(name.to_string(), vec!["bash".to_string()]);
        Ok(())
    }

    fn new_window(
        &self,
        session: &str,
        window_name: Option<&str>,
        _target_index: Option<usize>,
    ) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        let windows = state
            .sessions
            .get_mut(session)
            .ok_or_else(|| anyhow!("Session '{}' not found", session))?;

        let name = window_name.unwrap_or("unnamed").to_string();
        windows.push(name);
        Ok(())
    }

    fn send_keys(&self, session: &str, window_index: usize, command: &[String]) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        if !state.sessions.contains_key(session) {
            return Err(anyhow!("Session '{}' not found", session));
        }
        state
            .commands_sent
            .push((session.to_string(), window_index, command.to_vec()));
        Ok(())
    }

    fn kill_session(&self, name: &str) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        if state.sessions.remove(name).is_none() {
            return Err(anyhow!("Session '{}' not found", name));
        }
        Ok(())
    }

    fn kill_window(&self, session: &str, window_name: &str) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        let windows = state
            .sessions
            .get_mut(session)
            .ok_or_else(|| anyhow!("Session '{}' not found", session))?;

        if let Some(pos) = windows.iter().position(|w| w == window_name) {
            windows.remove(pos);
            Ok(())
        } else {
            Err(anyhow!(
                "Window '{}' not found in session '{}'",
                window_name,
                session
            ))
        }
    }

    fn rename_window(&self, session: &str, window_index: usize, new_name: &str) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        let windows = state
            .sessions
            .get_mut(session)
            .ok_or_else(|| anyhow!("Session '{}' not found", session))?;

        if window_index >= windows.len() {
            return Err(anyhow!(
                "Window index {} out of range in session '{}'",
                window_index,
                session
            ));
        }

        windows[window_index] = new_name.to_string();
        Ok(())
    }

    fn attach_session(&self, name: &str) -> Result<()> {
        let state = self.state.lock().unwrap();
        if !state.sessions.contains_key(name) {
            return Err(anyhow!("Session '{}' not found", name));
        }
        Ok(())
    }
}
