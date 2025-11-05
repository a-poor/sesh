//! App's CLI code.

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    version,
    about = "A CLI tool for managing TMUX sessions with persistent configuration",
    long_about = "sesh manages TMUX sessions defined in a .seshconf.toml file, allowing you to quickly start, stop, and manage multi-window terminal environments."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Path to the session configuration file
    #[arg(long, global = true, default_value = ".seshconf.toml")]
    pub config: PathBuf,

    /// Suppress output messages
    #[arg(short, long, global = true, action)]
    pub quiet: bool,
}

impl Cli {
    pub fn config_file_exists(&self) -> bool {
        self.config.exists()
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize a new session configuration file
    Init(InitArgs),

    /// Check the status of the session and its windows
    Status,

    /// Start the TMUX session and all configured windows
    Up,

    /// Stop the TMUX session
    Down,

    /// Start the session and attach to it (selects default window if configured)
    Attach,

    /// Restart the session (runs down then up)
    Restart,

    /// Manage windows in the session configuration
    Window(WindowArgs),
}

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Session name for config file.
    ///
    /// Defaults to random memorable text but a
    /// future version may use the dir name.
    pub name: Option<String>,

    /// Overwrite existing file if it already exists.
    #[arg(long, action)]
    pub overwrite: bool,
}

#[derive(Debug, Args)]
pub struct WindowArgs {
    #[command(subcommand)]
    pub command: WindowCommands,
}

#[derive(Debug, Subcommand)]
pub enum WindowCommands {
    /// Add a new window to the session configuration
    Add(WindowAddArgs),

    /// Remove a window from the session configuration
    Remove(WindowRemoveArgs),
}

#[derive(Debug, Args)]
pub struct WindowAddArgs {
    /// Optional name of the command
    ///
    /// Defaults to `cmd`
    #[arg(short, long)]
    pub name: Option<String>,

    /// Name of the command to run
    pub cmd: String,

    /// Arguments to the command
    pub args: Vec<String>,
}

#[derive(Debug, Args)]
pub struct WindowRemoveArgs {
    /// Name of the window to remove
    #[arg(short, long)]
    pub name: Option<String>,
}
