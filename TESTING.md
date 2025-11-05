# Testing Guide

This project has two types of tests: unit tests and integration tests.

## Unit Tests (Mock-based)

Unit tests use a `MockTmuxBackend` and don't require tmux to be installed. They run quickly and are ideal for CI/CD environments.

**Run unit tests:**
```bash
cargo test
```

These tests verify business logic without actually calling tmux commands.

## Integration Tests (Real tmux)

Integration tests use the actual tmux binary (`RealTmuxBackend`) and are marked with `#[ignore]` so they don't run by default. These tests will create and destroy real tmux sessions.

**Prerequisites:**
- tmux must be installed on your system
- Tests create sessions with names like `sesh-test-<timestamp>` to avoid conflicts

**Run integration tests:**
```bash
# Run only the integration tests
cargo test -- --ignored

# Run both unit and integration tests
cargo test -- --include-ignored
```

**Available integration tests:**
- `test_real_tmux_status_session_not_running` - Verify status when session doesn't exist
- `test_real_tmux_status_session_running` - Verify status when session exists
- `test_real_tmux_up_creates_new_session` - Test session creation
- `test_real_tmux_up_with_multiple_windows` - Test multi-window session creation
- `test_real_tmux_up_idempotent` - Test that running `up` twice is safe
- `test_real_tmux_down_kills_session` - Test session cleanup
- `test_real_tmux_down_nonexistent_session` - Test graceful handling of missing sessions
- `test_real_tmux_full_lifecycle` - Test complete up -> status -> down cycle

**Note:** Integration tests automatically clean up after themselves, but if a test fails, you may need to manually kill leftover sessions:

```bash
# List all tmux sessions
tmux ls

# Kill a specific test session
tmux kill-session -t sesh-test-<timestamp>
```

## Test Architecture

### MockTmuxBackend
- In-memory state tracking
- Thread-safe with `Arc<Mutex<>>`
- No external dependencies
- Fast execution
- Used by default unit tests

### RealTmuxBackend
- Executes actual tmux commands via `std::process::Command`
- Used by integration tests (behind `#[ignore]`)
- Requires tmux to be installed
- Creates real sessions on the system

Both backends implement the same `TmuxBackend` trait, allowing us to test the same business logic with different backends.

## CI/CD Considerations

For CI/CD pipelines:

1. **Default behavior** (`cargo test`) only runs unit tests - no tmux required
2. **Optional step** (`cargo test -- --ignored`) runs integration tests if tmux is available
3. Integration tests are safe to run in parallel due to unique session names

Example GitHub Actions workflow:

```yaml
- name: Run unit tests
  run: cargo test

- name: Install tmux
  run: sudo apt-get install -y tmux

- name: Run integration tests
  run: cargo test -- --ignored
```
