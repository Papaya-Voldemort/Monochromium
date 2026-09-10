# Monochromium

> A fast, local first CLI note taking app

![Monochromium](banner.png)

Monochromium lets you capture ideas, tasks, and mental health check-ins all without leaving the terminal!

## Why Monochromium?

Most note-taking apps are clunky, require opening another app, and are not built for developers in the slightest.

Monochromium is crazy light, modern, and allows for blazing fast workflows fully in plain text!

## Features
- Blazing-Fast, Small Binary
- Multiple Note Types
- Search and Filtering
- Fully Local Notes with SQLite
- Human-Readable Terminal Output

## Install

### From Source

> Requires Rust
```bash
git clone https://github.com/Papaya-Voldemort/Monochromium
cd Monochromium
cargo install --path .
```
### From Cargo (Recommended)

> Requires Rust
```bash
cargo install monochromium
```

### From Github Binary (MacOS)
TODO: Add link to binary

### Verify
```bash
mono --version
```

## Usage

```bash
mono add "Ship v1.0"
mono checkin
mono list --today
```

## Contributions

> We are a very small team (of 1) so your PRs may take several days to review :)

Contributions are completely welcome and accepted.

Feel free to submit a PR or a Issue for anything!

For larger changes please make and link and issue to keep things more trackable.

## License
MIT

