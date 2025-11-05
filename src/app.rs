//! Application code.

use crate::cli::{Cli, InitArgs, WindowAddArgs, WindowRemoveArgs};
use crate::conf::{Config, WindowConf};
use crate::tmux;
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
pub fn run_status(cli: &Cli) -> Result<()> {
    tmux::check_tmux_available()?;

    let config = Config::load(&cli.config)?;

    let session_exists = tmux::has_session(&config.name)?;

    if !session_exists {
        if !cli.quiet {
            println!("Session '{}' is NOT running", config.name);
        }
        return Ok(());
    }

    if !cli.quiet {
        println!("Session '{}' is running", config.name);

        let running_windows = tmux::list_windows(&config.name)?;

        if config.window.is_empty() {
            println!("  No windows configured");
        } else {
            println!("  Windows:");
            for (idx, window_conf) in config.window.iter().enumerate() {
                let window_name = window_conf.name.as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("unnamed");

                // Check if this window is running (by name or index)
                let is_running = running_windows.iter().any(|w| w == window_name)
                    || (idx < running_windows.len() && window_name == "unnamed");

                let status = if is_running { "✓" } else { "✗" };
                println!("    {} {}", status, window_name);
            }
        }
    }

    Ok(())
}

/// Ensure the session + windows are running.
pub fn run_up(cli: &Cli) -> Result<()> {
    tmux::check_tmux_available()?;

    let config = Config::load(&cli.config)?;

    // Check if session already exists
    let session_exists = tmux::has_session(&config.name)?;

    if !session_exists {
        // Create new session (detached)
        tmux::new_session(&config.name, true)?;

        if !cli.quiet {
            println!("Created session '{}'", config.name);
        }
    }

    // Get list of existing windows
    let existing_windows = if session_exists {
        tmux::list_windows(&config.name)?
    } else {
        // A new session always has one default window (index 0)
        vec![]
    };

    // Create windows from config
    for (idx, window_conf) in config.window.iter().enumerate() {
        let window_name = window_conf.name.as_deref();

        // Check if window already exists
        let window_exists = if let Some(name) = window_name {
            existing_windows.iter().any(|w| w == name)
        } else {
            false
        };

        if window_exists {
            if !cli.quiet {
                println!("  Window '{}' already exists", window_name.unwrap());
            }
            continue;
        }

        // For the first window, we need to handle it differently
        if idx == 0 && !session_exists {
            // The session was just created with a default window at index 0
            // We can rename it if needed, but for now we'll just send keys
            if let Some(command) = &window_conf.command {
                tmux::send_keys(&config.name, idx, command)?;

                if !cli.quiet {
                    let name = window_name.unwrap_or("window 0");
                    println!("  Executed command in {}", name);
                }
            }
        } else {
            // Create new window
            tmux::new_window(&config.name, window_name, Some(idx))?;

            if !cli.quiet {
                let name = window_name.unwrap_or(&format!("window {}", idx));
                println!("  Created window '{}'", name);
            }

            // Execute command if specified
            if let Some(command) = &window_conf.command {
                tmux::send_keys(&config.name, idx, command)?;

                if !cli.quiet {
                    let name = window_name.unwrap_or(&format!("window {}", idx));
                    println!("  Executed command in {}", name);
                }
            }
        }
    }

    if !cli.quiet {
        println!("Session '{}' is up", config.name);
    }

    Ok(())
}

/// Kill the session + windows.
pub fn run_down(cli: &Cli) -> Result<()> {
    tmux::check_tmux_available()?;

    let config = Config::load(&cli.config)?;

    // Check if session exists
    let session_exists = tmux::has_session(&config.name)?;

    if !session_exists {
        if !cli.quiet {
            println!("Session '{}' is not running", config.name);
        }
        return Ok(());
    }

    // Kill the session
    tmux::kill_session(&config.name)?;

    if !cli.quiet {
        println!("Killed session '{}'", config.name);
    }

    Ok(())
}

/// Ensure the session + windows are running and
/// attach to the session.
pub fn run_attach(cli: &Cli) -> Result<()> {
    tmux::check_tmux_available()?;

    // First, ensure the session is up
    run_up(cli)?;

    // Load config to get session name
    let config = Config::load(&cli.config)?;

    // Attach to the session (this will block until user detaches)
    tmux::attach_session(&config.name)?;

    Ok(())
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
pub fn run_window_add(cli: &Cli, args: &WindowAddArgs) -> Result<()> {
    let mut config = Config::load(&cli.config)?;

    // Build command vector from cmd + args
    let mut command = vec![args.cmd.clone()];
    command.extend(args.args.clone());

    // Create window config
    let window_conf = WindowConf {
        name: args.name.clone(),
        command: Some(command),
    };

    // Add to config
    config.window.push(window_conf);

    // Write updated config
    config.write(&cli.config)?;

    if !cli.quiet {
        let name = args.name.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("unnamed");
        println!("Added window '{}' to config", name);
    }

    Ok(())
}

/// Remove a window from the session config
pub fn run_window_remove(cli: &Cli, args: &WindowRemoveArgs) -> Result<()> {
    let mut config = Config::load(&cli.config)?;

    if let Some(name) = &args.name {
        // Find and remove window by name
        let initial_len = config.window.len();
        config.window.retain(|w| {
            w.name.as_ref().map(|n| n != name).unwrap_or(true)
        });

        if config.window.len() == initial_len {
            return Err(anyhow!("Window '{}' not found in config", name));
        }

        // Write updated config
        config.write(&cli.config)?;

        if !cli.quiet {
            println!("Removed window '{}' from config", name);
        }
    } else {
        return Err(anyhow!("Must specify --name to remove a window"));
    }

    Ok(())
}
