# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`sesh` is a CLI tool for managing TMUX sessions with persistent configuration. It allows users to define sessions with multiple windows/panes in a `.seshconf.toml` file and quickly start/stop/restart those sessions.

## Build and Development Commands

### Building
```bash
cargo build                    # Development build
cargo build --release          # Optimized release build
```

### Running
```bash
cargo run -- <command>         # Run the CLI with arguments
cargo run -- init              # Example: initialize config
cargo run -- --help            # View available commands
```

### Testing
```bash
cargo test                     # Run all tests
cargo test -- --nocapture      # Run tests with stdout visible
cargo test <test_name>         # Run specific test
```

### Code Quality
```bash
cargo clippy                   # Lint code
cargo fmt                      # Format code
cargo check                    # Fast syntax check without building
```

## Architecture

### Module Structure

- **main.rs**: Entry point that parses CLI args and dispatches to app handlers
- **cli.rs**: CLI definition using clap with derive macros. Defines `Cli`, `Command`, and all argument structs
- **app.rs**: Business logic for each command (init, status, up, down, attach, restart, window operations)
- **conf.rs**: Config file data structures (`Config`, `WindowConf`) with TOML serialization/deserialization
- **words.rs**: Random name generation (Docker-style adjective-noun combinations)
- **adjectives.rs**, **nouns.rs**: Word lists for random name generation

### Configuration File Format

The tool uses `.seshconf.toml` (configurable via `--config` flag) with this structure:

```toml
name = "session-name"

[[window]]
name = "editor"
command = ["vim", "."]

[[window]]
name = "server"
command = ["npm", "run", "dev"]
```

The config is loaded/written using the `Config::load()` and `Config::write()` methods in conf.rs.

### CLI Flow

1. `main.rs` parses CLI using clap
2. Pattern matches on `Command` enum to dispatch to appropriate `run_*` function in `app.rs`
3. App functions receive `&Cli` reference to access global options (config path, quiet mode)
4. Errors propagate as `anyhow::Result` and are printed in main.rs before exiting with code 1

### Current Implementation Status

Only `sesh init` is currently implemented. The following commands return "Not implemented" errors:
- status, up, down, attach, window add, window remove

The `restart` command is implemented as `run_down() + run_up()` but will fail until those are implemented.

## Adding New Commands

1. Add command variant to `Command` enum in cli.rs
2. Add argument struct if needed (like `InitArgs`)
3. Add pattern match case in main.rs
4. Implement `run_<command>` function in app.rs
5. Add tests in conf.rs or create new test module

## Testing Patterns

Tests are defined inline using `#[cfg(test)]` modules (see conf.rs:39-90). The test in conf.rs verifies TOML deserialization matches the expected `Config` struct.
