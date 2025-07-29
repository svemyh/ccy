#!/bin/bash

# CCO (Copy Command Output) - Bash hook script
# This script captures command outputs for the cco tool

# Function to generate consistent session ID (matches Rust implementation)
_cco_get_session_id() {
    local shell_pid="${PPID:-unknown}"
    local tty_info
    
    # Try to get TTY info
    if tty_info=$(tty 2>/dev/null) && [[ "$tty_info" != "not a tty" ]] && [[ -n "$tty_info" ]]; then
        tty_info="${tty_info//\/dev\//}"
        tty_info="${tty_info//\//_}"
    else
        # Fallback to shell level or other indicators
        tty_info="${BASH_SUBSHELL:-${SHLVL:-fallback}}"
    fi
    
    # Generate consistent session ID
    echo "session_${shell_pid}_${tty_info}" | tr -cd '[:alnum:]_'
}

# Function to capture command output after execution
_cco_capture_output() {
    local exit_code=$?
    
    # Skip if no previous command or if it's cco itself
    if [[ -z "$_cco_last_command" ]] || [[ "$_cco_last_command" =~ ^cco ]]; then
        return $exit_code
    fi
    
    # Skip if the command is interactive (basic heuristic)  
    if [[ "$_cco_last_command" =~ ^(vim|nano|emacs|less|more|man|top|htop|ssh|git\ log|git\ diff)$ ]]; then
        return $exit_code
    fi
    
    # Skip internal commands
    if [[ "$_cco_last_command" =~ ^(_cco_|PROMPT_COMMAND|source|export|alias|unalias|cd|pwd|echo|printf|test|\[|\]|builtin|command|type|which|history) ]]; then
        return $exit_code
    fi
    
    # Use a simpler approach: capture the last command's output by re-running it
    # This is not perfect but works for many common cases
    if command -v cco-capture >/dev/null 2>&1; then
        # For commands that are safe to re-run and produce consistent output
        if [[ "$_cco_last_command" =~ ^(ls|cat|grep|find|pwd|whoami|date|df|free|ps|uname) ]]; then
            # Re-execute the command and capture output
            local output
            output=$(eval "$_cco_last_command" 2>&1) || true
            echo "$output" | cco-capture "$_cco_last_command" 2>/dev/null || true
        else
            # For other commands, just store the command with empty output
            echo "" | cco-capture "$_cco_last_command" 2>/dev/null || true
        fi
    fi
    
    return $exit_code
}

# Set up the hook
if [[ -n "$BASH_VERSION" ]]; then
    # Use DEBUG trap to capture commands before execution
    _cco_debug_trap() {
        local cmd="${BASH_COMMAND}"
        
        # Skip our own functions and prompt command
        if [[ ! "$cmd" =~ ^(_cco_|PROMPT_COMMAND) ]]; then
            _cco_last_command="$cmd"
        fi
    }
    
    # Set the DEBUG trap
    trap '_cco_debug_trap' DEBUG
    
    # Add to PROMPT_COMMAND to process captured output after execution
    if [[ "$PROMPT_COMMAND" != *"_cco_capture_output"* ]]; then
        PROMPT_COMMAND="_cco_capture_output; ${PROMPT_COMMAND}"
    fi
fi