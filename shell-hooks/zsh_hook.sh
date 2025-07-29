#!/bin/zsh

# CCO (Copy Command Output) - Zsh hook script
# This script captures command outputs for the cco tool

# Function to capture command output
_cco_capture() {
    local exit_code=$?
    
    # Skip if no previous command or if it's cco itself
    if [[ -z "$_cco_last_command" ]] || [[ "$_cco_last_command" =~ ^cco ]]; then
        return $exit_code
    fi
    
    # Skip if the command is interactive (basic heuristic)
    if [[ "$_cco_last_command" =~ ^(vim|nano|emacs|less|more|man|top|htop|ssh)$ ]]; then
        return $exit_code
    fi
    
    if command -v cco-capture >/dev/null 2>&1; then
        # Store just the command for now
        # TODO: Implement proper output capture mechanism
        echo "" | cco-capture "$_cco_last_command" 2>/dev/null || true
    fi
    
    return $exit_code
}

# Function to capture command before execution
_cco_preexec() {
    _cco_last_command="$1"
}

# Set up zsh hooks
if [[ -n "$ZSH_VERSION" ]]; then
    # Use preexec to capture the command
    autoload -Uz add-zsh-hook
    add-zsh-hook preexec _cco_preexec
    
    # Use precmd to capture output after execution
    add-zsh-hook precmd _cco_capture
fi