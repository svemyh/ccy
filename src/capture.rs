use dirs::cache_dir;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

mod session;

#[derive(Serialize, Deserialize, Debug)]
struct CommandOutput {
    command: String,
    output: String,
    timestamp: u64,
    session_id: String,
}

fn get_cache_dir() -> io::Result<PathBuf> {
    let cache_dir = cache_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "No cache directory found")
    })?;
    let cco_dir = cache_dir.join("cco");
    fs::create_dir_all(&cco_dir)?;
    Ok(cco_dir)
}

fn get_session_id() -> io::Result<String> {
    Ok(session::get_session_id())
}

fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: cco-capture <command>");
        process::exit(1);
    }

    let command = args[1..].join(" ");
    
    // Read all input from stdin (the command output)
    let mut output = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut output) {
        eprintln!("Failed to read command output: {}", e);
        process::exit(1);
    }

    // Remove ANSI escape sequences to clean the output
    let cleaned_output = String::from_utf8_lossy(&strip_ansi_escapes::strip(&output)).to_string();

    let session_id = match get_session_id() {
        Ok(session_id) => session_id,
        Err(e) => {
            eprintln!("Failed to get session ID: {}", e);
            process::exit(1);
        }
    };

    let cache_dir = match get_cache_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to get cache directory: {}", e);
            process::exit(1);
        }
    };

    let cmd_output = CommandOutput {
        command: command.clone(),
        output: cleaned_output,
        timestamp: get_timestamp(),
        session_id: session_id.clone(),
    };

    let file_path = cache_dir.join(format!("{}.json", session_id));
    
    match serde_json::to_string_pretty(&cmd_output) {
        Ok(json_content) => {
            if let Err(e) = fs::write(&file_path, json_content) {
                eprintln!("Failed to write command output: {}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to serialize command output: {}", e);
            process::exit(1);
        }
    }
}