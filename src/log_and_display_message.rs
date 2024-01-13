extern crate native_dialog;

use native_dialog::MessageType;

use crate::construct_log_message;

pub fn log_and_display_message(message: &str, expected_folder_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Start log_and_display_message");
    
    // Construct the log message with the expected folder path
    let log_message = format!("{}\n\n{}", message, construct_log_message(expected_folder_path));


    // Display the log message in a pop-up dialog
    if let Err(err) = native_dialog::MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Information")
        .set_text(&log_message)
        .show_alert()
    {
        eprintln!("Failed to display message: {}", err);
    }
    println!("End log_and_display_message");

    Ok(())
}