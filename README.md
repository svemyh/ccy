# CCY (Console Command Yank)

A terminal utility that captures and yanks the output of your last command to the clipboard. Perfect for feeding command outputs into LLM agents or sharing terminal results.

## Features

- **Simple Usage**: Run `ccy` to yank your last command output to clipboard
- **Multi-Terminal Support**: Works across different terminal windows using PID-based session isolation  
- **Clean Output**: Strips ANSI escape sequences for clean text
- **Flexible Usage**: Yank to clipboard or print to stdout
- **Shell Support**: Works with bash and zsh
- **System-wide Installation**: Install once, use everywhere

## Installation

```bash
git clone <repository-url>
cd ccy
sudo ./install.sh  # Automatically enables CCY
```

## Usage

After installation (auto-enabled):

### Method 1: Manual Capture (Recommended)
```bash
# Run any command and manually capture its output
ls -la
echo "$(ls -la)" | ccy-capture "ls -la"

# Or use the 'run' wrapper function (if using simple hooks)
run ls -la  # Automatically captures output

# Then yank to clipboard
ccy
```

### Method 2: Basic Hook (Limited)
```bash
# Run any command normally (basic hook attempts capture)
ls -la

# Yank the output (command + output) to clipboard
ccy

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

## Storage

- Command outputs are stored in `~/.cache/ccy/`
- Each session gets its own file (e.g., `session_123_pts_0.json`)
- Files are overwritten with each new command (no accumulation)
- Clean text output (ANSI sequences stripped)

## Commands

- `ccy` - Main command to yank/print last output
- `ccy --enable` - Enable shell hooks
- `ccy --disable` - Disable shell hooks
- `ccy-capture <command>` - Manually capture command output (reads from stdin)
- `run <command>` - Wrapper to run command and auto-capture output

## Current Limitations

The automatic output capture (shell hooks) has limitations:

1. **Complex Hook Implementation**: Intercepting shell output is technically challenging
2. **Manual Capture Works Best**: For reliable capture, use `echo "$(command)" | ccy-capture "command"`
3. **Interactive Commands**: Automatically skips vim, nano, less, etc.
4. **Non-TTY Environments**: Limited functionality in environments without TTY

### Recommended Workflow

For best results, manually capture important command outputs:

```bash
# Instead of just: ls -la
# Use: 
output=$(ls -la)
echo "$output"
echo "$output" | ccy-capture "ls -la"
ccy  # Now yanks the captured output
```

Or use the `run` wrapper:
```bash
run ls -la  # Automatically captures and displays output
ccy         # Yanks the captured output
```

## Future Improvements (TODOs)

- **Better Shell Integration**: Improve automatic output capture mechanism
- **Visual Feedback**: Add confirmation when yanking to clipboard
- **Large Output Handling**: Add size limiting for very large outputs
- **Retroactive Capture**: Capture output from commands already run
- **More Shell Support**: Add fish and other shell support

## Requirements

- Rust/Cargo for building
- Linux (tested on Ubuntu 24.04)
- Clipboard utility (xclip, xsel, or wl-clipboard) for clipboard functionality
- Bash or Zsh shell

## Uninstall

```bash
ccy --disable  # Remove from shell config
sudo rm /usr/local/bin/ccy*
sudo rm -rf /etc/ccy
rm -rf ~/.cache/ccy
```

## License

MIT