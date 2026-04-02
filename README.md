# The `pathcheck` command.
Parse your `PATH` variable and check the status of each path.

# Installation
### Debian/Ubuntu based systems:
todo
### Arch based systems:
todo
### Cargo (build from source in any system)
```sh
cargo install pathcheck
```

This will install the `pathcheck` command in `$CARGO_HOME/bin/`.

> That folder should be on your `PATH` variable by the way.

To install elsewhere:
```sh
cargo install pathcheck --root /usr/local/bin
```

# Usage
```sh
$ pathcheck
OK  /home/{user}/.cargo/bin
OK  /home/{user}/.bun/bin
OK  /usr/local/sbin
OK  /usr/local/bin
OK  /usr/sbin
OK  /usr/bin
OK  /sbin
OK  /bin
OK  /usr/games
OK  /usr/local/games
ERR /snap/bin
```

The `-s` flag let's you change the status text style between plain text, nerd font icons, emojis, or none (equivalent to `-S`).

Use the `--color` flag to change color behavior.

Use the `--format` flag to specify a different output format.

**Run `path --help` for more information**.

### To simply display your paths:
```sh
pathcheck -S
```

### Exit codes
This program tries to exit with the code that corresponds to your system's standard. See `errno` for Linux-based systems and `net helpmsg` for Windows.

Exit code 0 means no errors.

# Changelog
## v0.1.0
- Release.
## v0.2.0
- Fixed a bug when exiting with a "not a directory" status code.
## v1.0.0
- Added formatting with the `--format` flag.

# TODO
- Add path description text
