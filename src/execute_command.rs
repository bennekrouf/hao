use std::io;
use std::process::Command;


pub fn execute_command(script_name: &str) -> Result<String, io::Error> {
    // Construct the path to the script
    let script_path = format!("scripts/{}", script_name);

    // Execute the script using 'bash' and capture the output
    let output = Command::new("bash")
        .arg(script_path)
        .output()?;

    // Check if the command was successful
    if !output.status.success() {
        // You can handle the error more specifically here if needed
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to execute script"));
    }

    // Convert the output (stdout) to a String
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();

    // Return the output string
    Ok(output_str)
}
