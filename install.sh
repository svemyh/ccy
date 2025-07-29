#!/bin/bash

# CCO (Copy Command Output) Installation Script

set -e

CCO_VERSION="0.1.0"
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="/etc/cco"
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
    log_info "Building CCO binaries..."
    cargo build --release
    
    if [[ ! -f "target/release/cco" ]] || [[ ! -f "target/release/cco-capture" ]]; then
        log_error "Build failed - binaries not found"
        exit 1
    fi
}

install_binaries() {
    log_info "Installing binaries to $INSTALL_DIR..."
    sudo cp target/release/cco "$INSTALL_DIR/"
    sudo cp target/release/cco-capture "$INSTALL_DIR/"
    sudo chmod +x "$INSTALL_DIR/cco" "$INSTALL_DIR/cco-capture"
}

install_shell_hooks() {
    log_info "Installing shell hooks to $SHELL_HOOKS_DIR..."
    sudo mkdir -p "$SHELL_HOOKS_DIR"
    sudo cp shell-hooks/bash_hook.sh "$SHELL_HOOKS_DIR/"
    sudo cp shell-hooks/zsh_hook.sh "$SHELL_HOOKS_DIR/"
    sudo chmod +x "$SHELL_HOOKS_DIR"/*.sh
}

create_enable_script() {
    log_info "Creating enable/disable scripts..."
    
    sudo tee "$INSTALL_DIR/cco-enable" > /dev/null << 'EOF'
#!/bin/bash

CONFIG_DIR="/etc/cco"
SHELL_HOOKS_DIR="$CONFIG_DIR/shell-hooks"

# Detect shell
if [[ -n "$BASH_VERSION" ]]; then
    SHELL_TYPE="bash"
    RC_FILE="$HOME/.bashrc"
    HOOK_FILE="$SHELL_HOOKS_DIR/bash_hook.sh"
elif [[ -n "$ZSH_VERSION" ]]; then
    SHELL_TYPE="zsh"
    RC_FILE="$HOME/.zshrc"
    HOOK_FILE="$SHELL_HOOKS_DIR/zsh_hook.sh"
else
    echo "Unsupported shell. CCO supports bash and zsh."
    exit 1
fi

# Check if already enabled
if grep -q "# CCO Hook" "$RC_FILE" 2>/dev/null; then
    echo "CCO is already enabled for $SHELL_TYPE"
    exit 0
fi

# Add hook to shell config
echo "Enabling CCO for $SHELL_TYPE..."
cat >> "$RC_FILE" << EOL

# CCO Hook - Copy Command Output
if [[ -f "$HOOK_FILE" ]]; then
    source "$HOOK_FILE"
fi
EOL

echo "CCO enabled! Restart your shell or run: source $RC_FILE"
EOF

    sudo tee "$INSTALL_DIR/cco-disable" > /dev/null << 'EOF'
#!/bin/bash

# Detect shell
if [[ -n "$BASH_VERSION" ]]; then
    RC_FILE="$HOME/.bashrc"
elif [[ -n "$ZSH_VERSION" ]]; then
    RC_FILE="$HOME/.zshrc"
else
    echo "Unsupported shell. CCO supports bash and zsh."
    exit 1
fi

# Remove CCO hook from shell config
if [[ -f "$RC_FILE" ]]; then
    # Create a backup
    cp "$RC_FILE" "$RC_FILE.cco-backup"
    
    # Remove CCO hook section
    sed -i '/# CCO Hook - Copy Command Output/,/^$/d' "$RC_FILE"
    
    echo "CCO disabled! Restart your shell to apply changes."
    echo "Backup saved as: $RC_FILE.cco-backup"
else
    echo "Shell config file not found: $RC_FILE"
fi
EOF

    sudo chmod +x "$INSTALL_DIR/cco-enable" "$INSTALL_DIR/cco-disable"
}

create_cache_dir() {
    log_info "Setting up cache directory..."
    # Cache directory will be created automatically by the application
    # in ~/.cache/cco/ for each user
}

main() {
    echo "CCO (Copy Command Output) Installer v$CCO_VERSION"
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
    create_enable_script
    create_cache_dir
    
    echo
    log_info "Installation completed successfully!"
    log_info "To enable CCO in your current shell, run: cco-enable"
    log_info "To disable CCO later, run: cco-disable"
    echo
    log_info "Usage:"
    log_info "  cco        - Copy last command output to clipboard"
    log_info "  cco -p     - Print last command output to terminal"
    log_info "  cco -c     - Copy only the command (not output)"
    log_info "  cco -o     - Copy only the output (not command)"
}

main "$@"