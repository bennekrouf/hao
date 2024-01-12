extern crate tui;
extern crate crossterm;

use std::io::stdout;
use std::process::Command;
use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders, List, ListItem};
use tui::style::{Style, Modifier, Color};
use crossterm::{execute, terminal::{self, ClearType}, event::{self, Event, KeyCode}};
use std::fs;

fn list_script_files(directory: &str) -> io::Result<Vec<String>> {
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

fn main() -> Result<(), io::Error> {
    let scripts = list_script_files("scripts")?;

    let mut choices = scripts;
    choices.push("Exit".to_string());

    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal::enable_raw_mode()?;

    // Clear the terminal before drawing the UI
    execute!(terminal.backend_mut(), terminal::Clear(ClearType::All))?;

    // let choices = ["Enable Proxy", "Disable Proxy", "Exit"];
    let mut current_selection = 0;

    loop {
        let items: Vec<ListItem> = choices.iter().enumerate()
            .map(|(i, choice)| {
                // Formatting the choice with an incremented number
                let display_text = format!("{:2}. {}", i + 1, choice);

                // Setting the style with colors
                let style = if i == current_selection {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                ListItem::new(display_text).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().title("Menu").borders(Borders::ALL));

        terminal.draw(|f| {
            let size = f.size();
            f.render_widget(list, size);
        })?;

        match event::read()? {
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => {
                        current_selection = (current_selection + 1) % choices.len();
                    }
                    KeyCode::Up => {
                        if current_selection > 0 {
                            current_selection -= 1;
                        } else {
                            current_selection = choices.len() - 1;
                        }
                    }
                    KeyCode::Enter => {
                        if choices[current_selection] == "Exit" {
                            break; // Exit if "Exit" is selected
                        } else {
                            execute_command(&choices[current_selection])?;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::Clear(ClearType::All))?;
    Ok(())
}

fn execute_command(script_name: &str) -> Result<(), io::Error> {
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