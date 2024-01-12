use std::io;
use std::process::Command;
use crossterm::event::{self};

pub fn execute_command(script_name: &str) -> Result<(), io::Error> {
    // Construct the path to the script
    let script_path = format!("scripts/{}", script_name);

    // Execute the script using 'bash'
    let output = Command::new("bash")
        .arg(script_path)
        .output()?;

    // Optionally, print the output for debugging
    println!("Status: {}", output.status);
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to execute script"));
    }

    // Wait for user input to continue
    println!("\nPress any key to continue...");
    event::read()?;
    Ok(())
}