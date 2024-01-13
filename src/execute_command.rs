use std::io;
use std::process::Command;
use std::path::Path;
use crate::log_and_display_message::log_and_display_message;

pub fn execute_command(script_name: &str, script_folder_path: &str) -> Result<String, io::Error> {
    println!("Start execute_command");
    
    // Check if the script folder exists
    let script_folder = Path::new(script_folder_path);
    println!("Passing let script_folder = Path::new(script_folder_path)");
    if !script_folder.exists() || !script_folder.is_dir() {
        let error_message = format!(
            "Script folder '{}' does not exist or is not a directory.",
            script_folder_path
        );
        
        // Call log_and_display_message with the error message and expected folder path
        if let Err(err) = log_and_display_message(&error_message, &format!("Expected folder path: {}", script_folder_path)) {
            eprintln!("Error in log_and_display_message: {}", err);
        }
    
        return Err(io::Error::new(io::ErrorKind::NotFound, error_message));
    }

    // Construct the path to the script
    let script_path = format!("{}/{}", script_folder_path, script_name);

    // Execute the script using 'bash' and capture the output and error
    let output = Command::new("bash")
        .arg(&script_path)
        .output()?;

    // Check if the command was successful
    if !output.status.success() {
        // Include the stderr in the error message
        let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();
        let error_message = format!(
            "Failed to execute script '{}'. Error: {}\nScript Path: {}",
            script_name, stderr_str, script_path
        );
        return Err(io::Error::new(io::ErrorKind::Other, error_message));
    }

    // Convert the stdout to a String
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    println!("process_script_folder after");
    println!("End execute_command");

    // Return the output string
    Ok(output_str)
}
