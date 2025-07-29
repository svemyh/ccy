use clap::{Arg, Command};
use dirs::cache_dir;
use std::fs;
use std::path::PathBuf;
use std::process;
use thiserror::Error;

mod session;

#[derive(Error, Debug)]
pub enum CcoError {
    #[error("No cache directory found")]
    NoCacheDir,
    #[error("No recent command output found")]
    NoRecentOutput,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[cfg(feature = "clipboard")]
    #[error("Clipboard error: {0}")]
    Clipboard(String),
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct CommandOutput {
    command: String,
    output: String,
    timestamp: u64,
    session_id: String,
}

fn get_cache_dir() -> Result<PathBuf, CcoError> {
    let cache_dir = cache_dir().ok_or(CcoError::NoCacheDir)?;
    let cco_dir = cache_dir.join("cco");
    fs::create_dir_all(&cco_dir)?;
    Ok(cco_dir)
}

fn get_current_session_id() -> Result<String, CcoError> {
    Ok(session::get_session_id())
}

fn find_latest_output() -> Result<CommandOutput, CcoError> {
    let cache_dir = get_cache_dir()?;
    
    // First try to find output for current session
    if let Ok(session_id) = get_current_session_id() {
        let session_file = cache_dir.join(format!("{}.json", session_id));
        if session_file.exists() {
            let content = fs::read_to_string(&session_file)?;
            if let Ok(output) = serde_json::from_str::<CommandOutput>(&content) {
                return Ok(output);
            }
        }
    }
    
    // If no current session output, find the most recent across all sessions
    let mut latest_output: Option<CommandOutput> = None;
    let mut latest_timestamp = 0u64;
    
    for entry in fs::read_dir(&cache_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(output) = serde_json::from_str::<CommandOutput>(&content) {
                    if output.timestamp > latest_timestamp {
                        latest_timestamp = output.timestamp;
                        latest_output = Some(output);
                    }
                }
            }
        }
    }
    
    latest_output.ok_or(CcoError::NoRecentOutput)
}

#[cfg(feature = "clipboard")]
fn copy_to_clipboard(text: &str) -> Result<(), CcoError> {
    // Try using system clipboard utilities directly as fallback
    // This is more reliable than the Rust clipboard crate in some environments
    
    // Try xclip first with timeout handling
    let xclip_result = process::Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(text.as_bytes())?;
                drop(stdin); // Close stdin to signal end of input
            }
            // Don't wait for child to avoid hanging
            Ok(())
        });
    
    if xclip_result.is_ok() {
        return Ok(());
    }
    
    // Try xsel as fallback
    if let Ok(_) = process::Command::new("xsel")
        .arg("--clipboard")
        .arg("--input")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(text.as_bytes())?;
            }
            child.wait().map(|_| ())
        }) {
        return Ok(());
    }
    
    // Try wl-copy for Wayland
    if let Ok(_) = process::Command::new("wl-copy")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(text.as_bytes())?;
            }
            child.wait().map(|_| ())
        }) {
        return Ok(());
    }
    
    // If all direct methods fail, try the Rust clipboard crate as last resort
    use clipboard::{ClipboardContext, ClipboardProvider};
    let mut ctx: ClipboardContext = ClipboardProvider::new()
        .map_err(|e| CcoError::Clipboard(format!("Failed to create clipboard context: {}", e)))?;
    ctx.set_contents(text.to_owned())
        .map_err(|e| CcoError::Clipboard(format!("Failed to set clipboard contents: {}", e)))?;
    
    Ok(())
}

fn get_home_dir() -> String {
    use std::env;
    
    // Try SUDO_USER first if running under sudo
    if let Ok(sudo_user) = env::var("SUDO_USER") {
        if sudo_user != "root" {
            return format!("/home/{}", sudo_user);
        }
    }
    
    // Fall back to HOME env var
    env::var("HOME").unwrap_or_else(|_| "/tmp".to_string())
}

fn handle_enable() {
    use std::env;
    use std::fs::OpenOptions;
    use std::io::Write;
    
    // Detect shell - handle sudo environment
    let shell = if env::var("BASH_VERSION").is_ok() {
        ("bash", get_home_dir() + "/.bashrc")
    } else if env::var("ZSH_VERSION").is_ok() {
        ("zsh", get_home_dir() + "/.zshrc")
    } else {
        // If no shell version vars, try to detect from SHELL env var
        let shell_path = env::var("SHELL").unwrap_or_default();
        if shell_path.contains("bash") {
            ("bash", get_home_dir() + "/.bashrc")
        } else if shell_path.contains("zsh") {
            ("zsh", get_home_dir() + "/.zshrc")
        } else {
            eprintln!("Unsupported shell. CCO supports bash and zsh.");
            eprintln!("Current SHELL: {}", shell_path);
            process::exit(1);
        }
    };
    
    let (shell_name, rc_file) = shell;
    let hook_file = format!("/etc/cco/shell-hooks/{}_hook.sh", shell_name);
    
    // Check if already enabled
    if let Ok(content) = fs::read_to_string(&rc_file) {
        if content.contains("# CCO Hook - Copy Command Output") {
            println!("CCO is already enabled for {}", shell_name);
            return;
        }
    }
    
    // Add hook to shell config
    let hook_content = format!(
        "\n# CCO Hook - Copy Command Output\nif [[ -f \"{}\" ]]; then\n    source \"{}\"\nfi\n",
        hook_file, hook_file
    );
    
    match OpenOptions::new().create(true).append(true).open(&rc_file) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(hook_content.as_bytes()) {
                eprintln!("Failed to write to {}: {}", rc_file, e);
                process::exit(1);
            }
            println!("CCO enabled for {}! Restart your shell or run: source {}", shell_name, rc_file);
        }
        Err(e) => {
            eprintln!("Failed to open {}: {}", rc_file, e);
            process::exit(1);
        }
    }
}

fn handle_disable() {
    use std::env;
    
    // Detect shell - handle sudo environment
    let shell = if env::var("BASH_VERSION").is_ok() {
        ("bash", get_home_dir() + "/.bashrc")
    } else if env::var("ZSH_VERSION").is_ok() {
        ("zsh", get_home_dir() + "/.zshrc")
    } else {
        // If no shell version vars, try to detect from SHELL env var
        let shell_path = env::var("SHELL").unwrap_or_default();
        if shell_path.contains("bash") {
            ("bash", get_home_dir() + "/.bashrc")
        } else if shell_path.contains("zsh") {
            ("zsh", get_home_dir() + "/.zshrc")
        } else {
            eprintln!("Unsupported shell. CCO supports bash and zsh.");
            eprintln!("Current SHELL: {}", shell_path);
            process::exit(1);
        }
    };
    
    let (shell_name, rc_file) = shell;
    
    // Remove CCO hook from shell config
    if !std::path::Path::new(&rc_file).exists() {
        eprintln!("Shell config file not found: {}", rc_file);
        return;
    }
    
    match fs::read_to_string(&rc_file) {
        Ok(content) => {
            // Create backup
            let backup_file = format!("{}.cco-backup", rc_file);
            if let Err(e) = fs::write(&backup_file, &content) {
                eprintln!("Warning: Failed to create backup: {}", e);
            }
            
            // Remove CCO hook section
            let lines: Vec<&str> = content.lines().collect();
            let mut new_lines = Vec::new();
            let mut in_cco_section = false;
            
            for line in lines {
                if line.trim() == "# CCO Hook - Copy Command Output" {
                    in_cco_section = true;
                    continue;
                }
                if in_cco_section && line.trim().is_empty() && new_lines.last().map_or(false, |l: &&str| l.starts_with("fi")) {
                    in_cco_section = false;
                    continue;
                }
                if !in_cco_section {
                    new_lines.push(line);
                }
            }
            
            let new_content = new_lines.join("\n");
            match fs::write(&rc_file, new_content) {
                Ok(()) => {
                    println!("CCO disabled for {}! Restart your shell to apply changes.", shell_name);
                    println!("Backup saved as: {}", backup_file);
                }
                Err(e) => {
                    eprintln!("Failed to write updated config: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read {}: {}", rc_file, e);
            process::exit(1);
        }
    }
}

fn main() {
    let matches = Command::new("cco")
        .version("0.1.0")
        .about("Copy Command Output - copies the last terminal command output")
        .arg(
            Arg::new("print")
                .short('p')
                .long("print")
                .help("Print to stdout instead of copying to clipboard")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("command-only")
                .short('c')
                .long("command-only")
                .help("Only show the command, not the output")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("output-only")
                .short('o')
                .long("output-only")
                .help("Only show the output, not the command")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("enable")
                .long("enable")
                .help("Enable CCO shell hooks in current shell")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("disable")
                .long("disable")
                .help("Disable CCO shell hooks in current shell")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let print_mode = matches.get_flag("print");
    let command_only = matches.get_flag("command-only");
    let output_only = matches.get_flag("output-only");
    let enable_mode = matches.get_flag("enable");
    let disable_mode = matches.get_flag("disable");

    // Handle enable/disable modes
    if enable_mode {
        handle_enable();
        return;
    }
    
    if disable_mode {
        handle_disable();
        return;
    }

    match find_latest_output() {
        Ok(cmd_output) => {
            let text = if command_only {
                cmd_output.command
            } else if output_only {
                cmd_output.output
            } else {
                format!("{}\n{}", cmd_output.command, cmd_output.output)
            };

            if print_mode {
                print!("{}", text);
            } else {
                #[cfg(feature = "clipboard")]
                {
                    match copy_to_clipboard(&text) {
                        Ok(()) => {
                            eprintln!("Successfully copied to clipboard");
                        }
                        Err(e) => {
                            eprintln!("Failed to copy to clipboard: {}", e);
                            eprintln!("Output:");
                            print!("{}", text);
                            process::exit(1);
                        }
                    }
                }
                #[cfg(not(feature = "clipboard"))]
                {
                    print!("{}", text);
                }
            }
        }
        Err(CcoError::NoRecentOutput) => {
            // TODO: Better error handling
            eprintln!("No recent command output found");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}