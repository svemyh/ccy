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
    use clipboard::{ClipboardContext, ClipboardProvider};
    let mut ctx: ClipboardContext = ClipboardProvider::new()
        .map_err(|e| CcoError::Clipboard(format!("Failed to create clipboard context: {}", e)))?;
    ctx.set_contents(text.to_owned())
        .map_err(|e| CcoError::Clipboard(format!("Failed to set clipboard contents: {}", e)))?;
    Ok(())
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
        .get_matches();

    let print_mode = matches.get_flag("print");
    let command_only = matches.get_flag("command-only");
    let output_only = matches.get_flag("output-only");

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
                            // TODO: Add visual feedback
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