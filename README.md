# Path
Nicely display your `$PATH` variable,
optionally checking for possible errors.

# Installation
### Cargo
```sh
cargo install path-cmd
```

This will install the `path` command in `$HOME/.cargo/bin/`. <br />
That folder should be on your `$PATH` variable by the way.

To install elsewhere:
```sh
cargo install path-cmd --root /usr/local/bin
```

# Usage
```sh
$ path
OK! "/home/{user}/.cargo/bin"
OK! "/home/{user}/.bun/bin"
OK! "/usr/local/sbin"
OK! "/usr/local/bin"
OK! "/usr/sbin"
OK! "/usr/bin"
OK! "/sbin"
OK! "/bin"
OK! "/usr/games"
OK! "/usr/local/games"
ERR "/snap/bin"
```

By default, `path` displays the "status" of the path (in my case, `/snap/bin` does not exist), followed by the path between quotes. This is because your `PATH` may contain invalid data, even empty strings. You can disable this by using the `-Q` flag as follows:

```sh
$ path -Q
OK! /home/{user}/.cargo/bin
OK! /home/{user}/.bun/bin
OK! /usr/local/sbin
OK! /usr/local/bin
OK! /usr/sbin
OK! /usr/bin
OK! /sbin
OK! /bin
OK! /usr/games
OK! /usr/local/games
ERR /snap/bin
```

The `-e` flag enables enumeration, and the `-z` flag enables padding with zeroes instead of using spaces.

The `-s` flag let's you change the status style between text, nerd font icons, emojis, or none (equivalent to `-S`) <br />
The `-S` flag disables status calculation.

Use the `-c` flag to change color behavior. Possible values are `always`, `never`, `auto` (default). <br />
The `-C` flag disables coloring.

**__All flags have their corresponding long form if you want to use them. Run `path -h` for more information.__**

### To simply display your paths:
```sh
$ path -QS
/home/{user}/.cargo/bin
/home/{user}/.bun/bin
/usr/local/sbin
/usr/local/bin
/usr/sbin
/usr/bin
/sbin
/bin
/usr/games
/usr/local/games
/snap/bin
```

### Exit codes
This program does its best to exit with the code that corresponds to your system's standard. See `errno` for Linux-based systems and `net helpmsg` for Windows. <br />
Exit code 0 always means no errors.

# Changelog
- 0.1.0: Release.
- 0.2.0: Fixed a little bug when exiting with a "not a directory" status code.

# TODO
- Organize with Unix/Windows folders.
