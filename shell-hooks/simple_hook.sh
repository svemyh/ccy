#!/bin/bash

# CCO (Copy Command Output) - Simple hook script
# This provides a 'run' wrapper function for capturing command output

# Function to run a command and capture its output for cco
run() {
    if [[ $# -eq 0 ]]; then
        echo "Usage: run <command>"
        return 1
    fi
    
    # Build the full command string
    local cmd="$*"
    
    # Execute the command and capture both stdout and stderr
    local output
    output=$("$@" 2>&1)
    local exit_code=$?
    
    # Print the output so user sees it
    echo "$output"
    
    # Store for cco if the capture binary exists
    if command -v cco-capture >/dev/null 2>&1; then
        echo "$output" | cco-capture "$cmd" 2>/dev/null || true
    fi
    
    return $exit_code
}

# Function to capture last history command output
cco-last() {
    if [[ $# -eq 0 ]]; then
        echo "Usage: cco-last (run this right after a command to capture its output)"
        return 1
    fi
    
    # This is a placeholder - in reality, capturing previous output is complex
    # This function exists as a TODO for future improvement
    echo "TODO: Implement retroactive output capture"
    return 1
}

echo "CCO simple hooks loaded. Use 'run <command>' to capture output, or just use cco after commands."