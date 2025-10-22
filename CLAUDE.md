# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`sesh` is a CLI tool for managing TMUX sessions via configuration files. It allows users to define session layouts in `.sesh-conf.toml` files and quickly restart/manage their tmux sessions with multiple windows and commands.

## Development Commands

### Build and Run
```bash
cargo build              # Build the project
cargo build --release    # Build optimized release binary
cargo run                # Run the project
cargo run -- <args>      # Run with arguments
```

### Testing
```bash
cargo test               # Run all tests
cargo test <test_name>   # Run a specific test
cargo test -- --nocapture  # Run tests with output visible
```

### Code Quality
```bash
cargo check              # Fast compilation check
cargo clippy             # Run linter
cargo fmt                # Format code
cargo fmt -- --check     # Check formatting without modifying
```

## Architecture

### Module Structure

- **`main.rs`**: Entry point (currently minimal, prints "Hello, world!")
- **`cli.rs`**: CLI argument parsing and command definitions (using clap)
- **`app.rs`**: Core application logic for session management
- **`conf.rs`**: Configuration file parsing and validation (`.sesh-conf.toml` files)
- **`db.rs`**: Database layer for tracking session state (using rusqlite)

### Configuration File Format

Sessions are defined in `.sesh-conf.toml` files with the following structure:

```toml
name = "session-name"

[[window]]
name = "window-name"
command = ["command", "args"]
depends_on = "other-window"  # Optional dependency
```

### Planned Commands

- `sesh init` - Create a new `.sesh-conf.toml` in the current directory
- `sesh up` - Start/ensure session is running with all windows
- `sesh down` - Shut down the session
- `sesh restart` - Stop and restart the session
- `sesh status` - Check current session status
- `sesh attach` - Ensure session is running and attach to it

### Key Dependencies

- **clap**: CLI argument parsing (with derive feature)
- **toml**: TOML configuration file parsing
- **serde**: Serialization/deserialization
- **rusqlite**: SQLite database for session state
- **validator**: Configuration validation (with derive feature)

## Implementation Notes

The project is in early development stage. The core modules are defined but mostly empty. When implementing features:

1. Session state should be tracked in SQLite (via `db.rs`)
2. TMUX interaction will need shell commands for creating sessions/windows and running commands
3. Window dependencies (e.g., `depends_on`) need to be resolved before starting windows
4. Configuration validation should use the `validator` crate to ensure valid TOML structures
