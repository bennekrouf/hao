use std::error::Error;

use crate::{list_script_files::list_script_files, log_and_display_message::{log_and_display_message, self}};

pub fn process_script_folder(script_folder_path: &str) -> Result<(), Box<dyn Error>> {
    let scripts = list_script_files(script_folder_path)?;

    let mut choices = scripts;

    choices.push("Exit".to_string());

    let current_dir = std::env::current_dir()?;
    let scripts_folder_relative_path = "scripts";
    let scripts_folder_absolute_path = current_dir.join(scripts_folder_relative_path);

    log_and_display_message::log_and_display_message(
        "This is a custom log message.",
        &scripts_folder_absolute_path.display().to_string(),
    )?;
    Ok(())
}