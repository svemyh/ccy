use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    println!("Testing clipboard functionality...");
    
    match ClipboardProvider::new() {
        Ok(mut ctx) => {
            println!("Clipboard context created successfully");
            
            let test_text = "Hello clipboard test!";
            match ctx.set_contents(test_text.to_owned()) {
                Ok(()) => {
                    println!("Successfully set clipboard contents");
                    
                    // Try to read it back
                    match ctx.get_contents() {
                        Ok(contents) => {
                            println!("Retrieved from clipboard: '{}'", contents);
                        }
                        Err(e) => {
                            println!("Failed to retrieve from clipboard: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to set clipboard contents: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to create clipboard context: {}", e);
        }
    }
}