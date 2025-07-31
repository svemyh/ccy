# ccy (Console Command Yank)

TL;DR: `ccy` captures and yanks the last terminal command output to clipboard

[![Crates.io](https://img.shields.io/crates/v/ccy)](https://crates.io/crates/ccy)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Perfect for feeding command outputs into LLM agents or sharing terminal results at blazing speed.

## Features

- **Simple Usage**: Run `ccy` to yank your last command output in the terminal to clipboard
- **Multi-Terminal Support**: Works across different terminal windows using PID-based session isolation  
- **Blazing Fast**: Rust-based implementation for maximum performance 
- **Support**: Works with bash and zsh, and across clipboard utilities (xclip, xsel, wl-copy)

## Installation

(Requires [Rust/Cargo](https://www.rust-lang.org/))

### From Crates.io (Recommended)

```bash
# Install ccy
cargo install ccy

# Enable shell hooks (required for automatic capture)
ccy --enable

# Restart your shell or run: source ~/.bashrc
```

### From Source

```bash
git clone https://github.com/svemyh/ccy
cd ccy
sudo ./install.sh  # Automatically enables shell hooks
```

## Usage

#### Basic Usage:

```bash
ls -la
ccy  # Automatically yanks the output of 'ls -la' to clipboard
```

#### Additional Options:
```bash
# Print to terminal instead of clipboard  
ccy -p

# Yank only the command
ccy -c

# Yank only the output
ccy -o
```

## How It Works

1. **PID-based Storage**: Each terminal session is identified by its PID and stores outputs in `~/.cache/ccy/`
2. **JSON Storage**: Command and output stored as JSON (e.g., `session_123_pts_0.json`)
3. **Latest Retrieval**: The `ccy` command reads the most recent output from your current terminal session

### Storage

- Command outputs are stored in `~/.cache/ccy/`
- Each session gets its own file (e.g., `session_123_pts_0.json`)
- Files are overwritten with each new command (no accumulation)
- Clean text output (ANSI sequences stripped)

## Main Commands

- `ccy` - Main command to yank/print last output
- `ccy --help` - Show help and additional commands
- `ccy --enable` - Enable shell hooks
- `ccy --disable` - Disable shell hooks
- `ccy-capture <command>` - Manually capture command output (reads from stdin)


## Future Improvements (TODOs)

- **Better Shell Integration**: Improve automatic output capture mechanism, terminal outputs from certain programs are not correctly parsed
- **More Shell Support**: Add fish and other shell support
- **More Clipboard Support**: Add more clipboard utilities (e.g., pbcopy, pbpaste)
- **Cross-Platform Support**: Add support for Windows and macOS

## Requirements

- Rust/Cargo for building
- Linux (tested on Ubuntu 24.04)
- **Clipboard utility** for clipboard functionality:
  - **xclip** (recommended): `sudo apt install xclip`
  - **xsel**: `sudo apt install xsel` 
  - **wl-copy** (Wayland): `sudo apt install wl-clipboard`
- Bash or Zsh shell

## Uninstall

```bash
ccy --disable  # Remove from shell config
sudo rm /usr/local/bin/ccy*
sudo rm -rf /etc/ccy
rm -rf ~/.cache/ccy
```

## License

[MIT](LICENSE)