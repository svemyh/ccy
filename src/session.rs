use std::env;
use std::process;

/// Generate a consistent session ID that works across shell hooks and main binary
pub fn get_session_id() -> String {
    // Use a combination of environment variables that should be consistent
    // within the same shell session
    
    // Method 1: Try to get shell-specific variables
    let shell_pid = env::var("PPID").unwrap_or_else(|_| "unknown".to_string());
    
    // Method 2: Try terminal-related environment variables
    let term_id = env::var("WINDOWID")
        .or_else(|_| env::var("TERM_SESSION_ID"))
        .or_else(|_| env::var("TMUX_PANE"))
        .unwrap_or_else(|_| "default".to_string());
    
    // Method 3: Get actual TTY if available (but handle "not a tty" gracefully)
    let tty_info = if let Ok(output) = process::Command::new("tty").output() {
        let tty = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !tty.is_empty() && tty != "not a tty" {
            tty.replace("/dev/", "").replace("/", "_")
        } else {
            // Fallback: use bash-specific variables if available
            env::var("BASH_SUBSHELL")
                .or_else(|_| env::var("SHLVL"))
                .unwrap_or_else(|_| "fallback".to_string())
        }
    } else {
        "no_tty".to_string()
    };
    
    // Create a deterministic session ID
    // Don't include the current process PID as that changes each time
    format!("session_{}_{}", shell_pid, tty_info)
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}