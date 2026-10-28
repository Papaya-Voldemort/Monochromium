# Monochromium

> A fast, local-first CLI note-taking app for developers

![Hackatime](https://hackatime.hackclub.com/api/v1/badge/U09197C6M2B/Papaya-Voldemort/Monochromium)
[![Crates.io](https://img.shields.io/crates/v/monochromium)](https://crates.io/crates/monochromium)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

![Monochromium](https://raw.githubusercontent.com/Papaya-Voldemort/Monochromium/master/images/banner.png)

## Why Monochromium?

Leaving the terminal is awful, it takes a couple seconds longer than it should, breaks your flow, and who knows if you
ever make it back.
That's where Monochromium comes in, it helps you stay in the terminal by providing a way to take notes right in it.

It's as simple as `mono add "hello world"`, and that's it. You're done.

![Demo](https://raw.githubusercontent.com/Papaya-Voldemort/Monochromium/master/images/demo.gif)

Notes are fully local and can be accessed by your normal terminal commands like `cat`, `nano`, `nvim` and any others!

Monochromium keeps your mental health in mind too. It helps you see patterns in your mood across days!

## Quick Install

Want to skip the reading? [Jump to Install](#Install).

## Features

Monochromium is packed with high-value features, check them out!

### Quick Commands

- `mono add "content"`: Add a note of any type!
- `mono checkin "content"`: Make a mental health check in
- `mono todo "content"`: Create a quick task
- `mono idea "content"`: Jot down your best ideas

### Fast Reads

- `mono view <id>`: View a specific note and its contents
- `mono list`: List the latest notes
- `mono search "query"`: Query your notes content and metadata!
- `mono export`: Export EVERYTHING including metadata in a nice toml format

### Powerful Flags

Try using flags like `-c` to copy the output to your clipboard, `-v` on list to see the contents of the listed notes,
and many more!

Run `mono help` to see the general help or `mono <command> --help` to see help for that specific command.

### Stats and Mood Tracking (beta)

Introducing the most powerful (beta) feature of Monochromium! The `mono stats` command!

Thought those values for mood tracking were dumb? NOPE this is what they are for! Behold the beauty!

![Stats](https://raw.githubusercontent.com/Papaya-Voldemort/Monochromium/master/images/stats.png)

Anyway it's super useful to track your mood over time in a super low friction way!

### Cryptography (Note Locking/Unlocking)

Try locking your notes with `mono lock <id>` and unlock them with `mono unlock <id>`!

We use **Argon2** and **chacha20poly1305** to securely lock your files. Metadata is NOT locked so keep that in mind.

Monochromium is only self reviewed for security and is not cryptographically perfect, notes may also show up in your
shell history so keep that in mind.

### Reminder and TODO List

Add a todo list and a reminder to make checkins to your shell!

`mono init` adds the following lines to your shell config to run on start:

```bash
# >>> monochromium >>>
mono reminder 2>/dev/null
# <<< monochromium <<<
```

## Install

### From Homebrew (recommended)

```bash
brew tap Papaya-Voldemort/tap
brew install Papaya-Voldemort/tap/monochromium
```

### From Cargo/Crates.io

> Requires Rust and Cargo

```bash
cargo install monochromium
```

> If `mono` is not found try running `exec $SHELL` to reload your shell!

### From Prebuilt Github Binary

Download the correct binary for your OS [here](https://github.com/Papaya-Voldemort/Monochromium/releases/latest)

### Verify

Current Latest Version: ![GitHub tag](https://img.shields.io/github/v/tag/Papaya-Voldemort/Monochromium)

```bash
mono --version
```

## Where your notes live

Your notes live in two places as `.md` files and SQLite entries.

The `.md` files are stored in a folder within your Documents folder.

The SQLite db handles the connecting file path and other metadata like the note type, title, etc.

## Contributions

> We are a very small team (of 1 person :sob:) so your PRs may take several days to review :)

Contributions are completely welcome and accepted.

Feel free to submit a PR or an Issue for anything!

For larger changes please make and link and issue to keep things more trackable.