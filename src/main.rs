extern crate tui;
extern crate crossterm;

pub mod execute_command;
pub mod list_script_files;

use crate::execute_command::execute_command;
use crate::list_script_files::list_script_files;

use std::io::stdout;

use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders, List, ListItem};
use tui::style::{Style, Modifier, Color};
use crossterm::{execute, terminal::{self, ClearType}, event::{self, Event, KeyCode}};


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