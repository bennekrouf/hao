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
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use tui::style::{Style, Modifier, Color};
use tui::layout::{Layout, Constraint, Direction};
use tui::text::{Spans, Span};
use tui::widgets::ListState;
use crossterm::{execute, terminal::{self, ClearType}, event::{self, Event, KeyCode}};

fn main() -> Result<(), io::Error> {
    let scripts = list_script_files("scripts")?;
    let mut choices = scripts;
    choices.push("Exit".to_string());

    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal::enable_raw_mode()?;

    let mut current_selection = 0;
    let mut scroll: u16 = 0;
    let mut output_log = String::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                ])
                .split(size);

            let items: Vec<ListItem> = choices.iter().enumerate()
                .map(|(i, choice)| {
                    let display_text = format!("{:2}. {}", i + 1, choice);
                    let style = if i == current_selection {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(Spans::from(Span::styled(display_text, style)))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().title("Commands").borders(Borders::ALL))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">> ");

            let mut state = ListState::default();
            state.select(Some(current_selection));
            f.render_stateful_widget(list, chunks[0], &mut state);

            let output = Paragraph::new(output_log.as_ref())
                .block(Block::default().title("Output Log").borders(Borders::ALL))
                .wrap(Wrap { trim: true });
            f.render_widget(output, chunks[1]);
        })?;

        match event::read()? {
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => {
                        current_selection = (current_selection + 1).min(choices.len() - 1);
                    }
                    KeyCode::Up => {
                        current_selection = current_selection.saturating_sub(1);
                    }
                    KeyCode::Enter => {
                        if choices[current_selection] == "Exit" {
                            break;
                        } else {
                            output_log = execute_command(&choices[current_selection])?;
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