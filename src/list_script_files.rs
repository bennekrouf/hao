use std::fs;
use std::io;

pub fn list_script_files(directory: &str) -> io::Result<Vec<String>> {
    let mut choices = Vec::new();
    let paths = fs::read_dir(directory)?;

    for path in paths {
        let path = path?.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("sh") {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                choices.push(file_name.to_string());
            }
        }
    }

    Ok(choices)
}
