//! Application code.

use crate::cli::{Cli, InitArgs};
use crate::conf::Config;
use crate::words::rand_phrase;
use anyhow::{Result, anyhow};

pub fn run_init(cli: &Cli, args: &InitArgs) -> Result<()> {
    if cli.config_file_exists() && !args.overwrite {
        return Err(anyhow!(
            "Config file {:?} already exists. To overwrite, pass --overwrite.",
            cli.config
        ));
    }

    let name = match args.name.as_ref() {
        Some(n) => n.clone(),
        None => rand_phrase(None, None)?,
    };
    let conf = Config {
        name,
        ..Default::default()
    };

    conf.write(&cli.config)?;

    if !cli.quiet {
        println!("Wrote config file to {:?}", &cli.config);
    }
    Ok(())
}

/// Check to see if the session is running and if
/// each of the session's windows are running.
pub fn run_status(_cli: &Cli) -> Result<()> {
    Err(anyhow!("Not implemented"))
}

/// Ensure the session + windows are running.
pub fn run_up(_cli: &Cli) -> Result<()> {
    Err(anyhow!("Not implemented"))
}

/// Kill the session + windows.
pub fn run_down(_cli: &Cli) -> Result<()> {
    Err(anyhow!("Not implemented"))
}

/// Ensure the session + windows are running and
/// attach to the session.
pub fn run_attach(_cli: &Cli) -> Result<()> {
    Err(anyhow!("Not implemented"))
}

/// Kill and re-start the session.
///
/// Shorthand for running `down` and then `up`.
pub fn run_restart(cli: &Cli) -> Result<()> {
    run_down(&cli)?;
    run_up(&cli)?;
    Ok(())
}

/// Add a window to the session config.
pub fn run_window_add(_cli: &Cli) -> Result<()> {
    Err(anyhow!("Not implemented"))
}

/// Remove a window to the session config
pub fn run_window_remove(_cli: &Cli) -> Result<()> {
    Err(anyhow!("Not implemented"))
}
