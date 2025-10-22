//! App's CLI code.

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(long, global = true, default_value = ".seshconf.toml")]
    pub config: PathBuf,

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
    Init(InitArgs),
    Status,
    Up,
    Down,
    Attach,
    Restart,
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
    Add(WindowAddArgs),
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
    #[arg(short, long)]
    pub name: Option<String>,
}
