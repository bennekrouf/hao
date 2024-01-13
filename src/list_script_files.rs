use std::fs;
use std::io;

use crate::log_and_display_message::log_and_display_message;

pub fn list_script_files(directory: &str) -> io::Result<Vec<String>> {
    println!("Start log_and_display_message");

    let mut choices = Vec::new();
    let paths = match fs::read_dir(directory) {
        Ok(paths) => paths,
        Err(err) => {
            // Call log_and_display_message with the error message and directory path
            let _ = log_and_display_message(&format!("Error: {}", err), directory);
            return Err(err);
        }
    };

    for path in paths {
        let path = path?.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("sh") {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                choices.push(file_name.to_string());
            }
        }
    }
    println!("End log_and_display_message");

    
    Ok(choices)
}
