# CCO (Copy Command Output)

A terminal utility that captures and copies the output of your last command to the clipboard. Perfect for feeding command outputs into LLM agents or sharing terminal results.

## Features

- **Simple Usage**: Run `cco` to copy your last command output to clipboard
- **Multi-Terminal Support**: Works across different terminal windows using TTY-based session isolation  
- **Clean Output**: Strips ANSI escape sequences for clean text
- **Flexible Usage**: Copy to clipboard or print to stdout
- **Shell Support**: Works with bash and zsh
- **System-wide Installation**: Install once, use everywhere

## Installation

```bash
git clone <repository-url>
cd cco
sudo ./install.sh
cco-enable  # Enable in current shell
```

## Usage

After installation and enabling:

### Method 1: Manual Capture (Recommended)
```bash
# Run any command and manually capture its output
ls -la
echo "$(ls -la)" | cco-capture "ls -la"

# Or use the 'run' wrapper function (if using simple hooks)
run ls -la  # Automatically captures output

# Then copy to clipboard
cco
```

### Method 2: Basic Hook (Limited)
```bash
# Run any command normally (basic hook attempts capture)
ls -la

# Copy the output (command + output) to clipboard
cco

# Print to terminal instead of clipboard  
cco -p

# Copy only the command
cco -c

# Copy only the output
cco -o
```

## How It Works

1. **TTY-based Storage**: Each terminal session is identified by its TTY and stores outputs in `~/.cache/cco/`
2. **JSON Storage**: Command and output stored as JSON (e.g., `pts_0.json`, `pts_1.json`)
3. **Latest Retrieval**: The `cco` command reads the most recent output from your current terminal session

## Storage

- Command outputs are stored in `~/.cache/cco/`
- Each TTY gets its own file (e.g., `pts_0.json`, `pts_1.json`)
- Files are overwritten with each new command (no accumulation)
- Clean text output (ANSI sequences stripped)

## Commands

- `cco` - Main command to copy/print last output
- `cco-enable` - Enable CCO in current shell
- `cco-disable` - Disable CCO in current shell
- `cco-capture <command>` - Manually capture command output (reads from stdin)
- `run <command>` - Wrapper to run command and auto-capture output

## Current Limitations

The automatic output capture (shell hooks) has limitations:

1. **Complex Hook Implementation**: Intercepting shell output is technically challenging
2. **Manual Capture Works Best**: For reliable capture, use `echo "$(command)" | cco-capture "command"`
3. **Interactive Commands**: Automatically skips vim, nano, less, etc.
4. **Non-TTY Environments**: Limited functionality in environments without TTY

### Recommended Workflow

For best results, manually capture important command outputs:

```bash
# Instead of just: ls -la
# Use: 
output=$(ls -la)
echo "$output"
echo "$output" | cco-capture "ls -la"
cco  # Now copies the captured output
```

Or use the `run` wrapper:
```bash
run ls -la  # Automatically captures and displays output
cco         # Copies the captured output
```

## Future Improvements (TODOs)

- **Better Shell Integration**: Improve automatic output capture mechanism
- **Visual Feedback**: Add confirmation when copying to clipboard
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
cco-disable  # Remove from shell config
sudo rm /usr/local/bin/cco*
sudo rm -rf /etc/cco
rm -rf ~/.cache/cco
```

## License

MIT