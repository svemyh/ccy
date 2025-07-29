# Changelog

All notable changes to CCY (Console Command Yank) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-07-30

### Added
- Initial release of CCY (Console Command Yank)
- Core functionality to capture and yank command outputs to clipboard
- PID-based session isolation for multi-terminal support
- Manual capture with `ccy-capture` command
- Clipboard integration with fallback to stdout
- Shell hooks for bash and zsh (basic implementation)
- Auto-enable on installation
- Built-in enable/disable commands (`ccy --enable`, `ccy --disable`)
- Support for multiple output modes (`-p`, `-c`, `-o`)
- Cross-platform clipboard support (xclip, xsel, wl-copy)
- Clean text output (ANSI sequences stripped)
- MIT license
- Comprehensive documentation

### Features
- `ccy` - Main command to yank last output to clipboard
- `ccy -p` - Print output to terminal instead of clipboard
- `ccy -c` - Yank only the command (not output)
- `ccy -o` - Yank only the output (not command)
- `ccy --enable` - Enable shell hooks in current shell (auto-enabled when using install.sh)
- `ccy --disable` - Disable shell hooks in current shell
- `ccy-capture <command>` - Manually capture command output

### Technical Details
- Written in Rust for performance and safety
- Uses serde for JSON serialization
- Supports bash and zsh shells
- Storage in `~/.cache/ccy/` directory
- Session files named by PID and TTY information