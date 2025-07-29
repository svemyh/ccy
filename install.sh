#!/bin/bash

# CCY (Console Command Yank) Installation Script

set -e

CCY_VERSION="0.1.0"
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="/etc/ccy"
SHELL_HOOKS_DIR="$CONFIG_DIR/shell-hooks"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_dependencies() {
    log_info "Checking dependencies..."
    
    if ! command -v cargo >/dev/null 2>&1; then
        log_error "Rust/Cargo is required but not installed."
        log_info "Install Rust from: https://rustup.rs/"
        exit 1
    fi
    
    # Check for clipboard utilities
    if command -v xclip >/dev/null 2>&1 || command -v xsel >/dev/null 2>&1 || command -v wl-clipboard >/dev/null 2>&1; then
        log_info "Clipboard utility found"
    else
        log_warn "No clipboard utility found. Install xclip, xsel, or wl-clipboard for clipboard support"
    fi
}

build_binaries() {
    log_info "Building CCY binaries..."
    cargo build --release
    
    if [[ ! -f "target/release/ccy" ]] || [[ ! -f "target/release/ccy-capture" ]]; then
        log_error "Build failed - binaries not found"
        exit 1
    fi
}

install_binaries() {
    log_info "Installing binaries to $INSTALL_DIR..."
    sudo cp target/release/ccy "$INSTALL_DIR/"
    sudo cp target/release/ccy-capture "$INSTALL_DIR/"
    sudo chmod +x "$INSTALL_DIR/ccy" "$INSTALL_DIR/ccy-capture"
}

install_shell_hooks() {
    log_info "Installing shell hooks to $SHELL_HOOKS_DIR..."
    sudo mkdir -p "$SHELL_HOOKS_DIR"
    sudo cp shell-hooks/bash_hook.sh "$SHELL_HOOKS_DIR/"
    sudo cp shell-hooks/zsh_hook.sh "$SHELL_HOOKS_DIR/"
    sudo chmod +x "$SHELL_HOOKS_DIR"/*.sh
}

auto_enable() {
    log_info "Auto-enabling CCY in current shell..."
    
    # Use the new ccy --enable command
    if command -v ccy >/dev/null 2>&1; then
        ccy --enable
    else
        log_warn "ccy command not found in PATH, skipping auto-enable"
    fi
}

create_cache_dir() {
    log_info "Setting up cache directory..."
    # Cache directory will be created automatically by the application
    # in ~/.cache/cco/ for each user
}

main() {
    echo "CCY (Console Command Yank) Installer v$CCY_VERSION"
    echo "================================================"
    
    if [[ $EUID -ne 0 ]] && [[ "$1" != "--user" ]]; then
        log_error "This script requires root privileges for system-wide installation."
        log_info "Run with sudo, or use --user for user-only installation (not implemented yet)"
        exit 1
    fi
    
    check_dependencies
    build_binaries
    install_binaries
    install_shell_hooks
    create_cache_dir
    auto_enable
    
    echo
    log_info "Installation completed successfully!"
    log_info "CCY has been automatically enabled in your current shell."
    log_info "To enable CCY in other shells, run: ccy --enable"
    log_info "To disable CCY later, run: ccy --disable"
    echo
    log_info "Usage:"
    log_info "  ccy           - Yank last command output to clipboard"
    log_info "  ccy -p        - Print last command output to terminal"
    log_info "  ccy -c        - Yank only the command (not output)"
    log_info "  ccy -o        - Yank only the output (not command)"
    log_info "  ccy --enable  - Enable shell hooks"
    log_info "  ccy --disable - Disable shell hooks"
}

main "$@"