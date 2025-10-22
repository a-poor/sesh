# sesh

My CLI for keeping track of TMUX sessions and restarting them where I left off.

## Explanation

The goal is to have a file that I can store in a directory (e.g. `.sesh-conf.toml`)
so I can quickly restart my tmux session where I left off.

The directory conf file might look like this:

```toml
# .sesh-conf.toml
name = "my-dir-name" # Current directory name or fun random word combo

[[window]]
name = "editor"
command = ["vim", "."]

[[window]]
name = "claude"
command = ["claude"]

[[window]]
name = "server"
command = ["npm", "run", "dev"]
depends_on = "db" # Maybe?

[[window]]
name = "db"
command = ["docker", "compose", "up", "db"]
```

Running `sesh init` could create that config file and set the name to the directory
name or set it to a random name (like docker random naming).

Then you can run `sesh up` and it will make sure there is a session with the given
name (in this case `my-dir-name`) and make sure all of the `[[window]]`s are
running, with the given names.

A later version could also include some options for layouts -- but I don't use that
much so it isn't my 1st priority.

There could be other commands like `sesh down` (to shut down the session),
`sesh restart` (to stop and restart the session), `sesh status` (to check the current
session's status), and `sesh attach` (to ensure the current session is running and
attach)

Maybe another future version could *freeze* the current state, by getting the session's
currently running commands (get the pane's PID and then use that with `ps` to get
the command?).

