#!/bin/zsh

# CCY (Console Command Yank) - Zsh hook script
# This script captures command outputs for the ccy tool

# Function to capture command output
_ccy_capture() {
    local exit_code=$?
    
    # Skip if no previous command or if it's cco itself
    if [[ -z "$_ccy_last_command" ]] || [[ "$_ccy_last_command" =~ ^ccy ]]; then
        return $exit_code
    fi
    
    # Skip if the command is interactive (basic heuristic)
    if [[ "$_cco_last_command" =~ ^(vim|nano|emacs|less|more|man|top|htop|ssh)$ ]]; then
        return $exit_code
    fi
    
    if command -v ccy-capture >/dev/null 2>&1; then
        # Store just the command for now
        # TODO: Implement proper output capture mechanism
        echo "" | ccy-capture "$_ccy_last_command" 2>/dev/null || true
    fi
    
    return $exit_code
}

# Function to capture command before execution
_ccy_preexec() {
    _ccy_last_command="$1"
}

# Set up zsh hooks
if [[ -n "$ZSH_VERSION" ]]; then
    # Use preexec to capture the command
    autoload -Uz add-zsh-hook
    add-zsh-hook preexec _ccy_preexec
    
    # Use precmd to capture output after execution
    add-zsh-hook precmd _ccy_capture
fi