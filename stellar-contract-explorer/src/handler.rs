use std::fmt;
use std::fmt::{Debug, Display};
use std::ptr::from_mut;
use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Paragraph, Widget};
use tokio::net::unix::uid_t;
use xshell::{cmd, Cmd, Shell};

use crate::app::{App, AppResult, ListStates};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Tab handlers
        KeyCode::Right => {
            app.next_tab();
        }
        KeyCode::Left => {
            app.previous_tab();
        }
        KeyCode::Up => {
            ListStates::select(app.selected_tab, &mut app.list_states).select_previous();
        }
        KeyCode::Down => {
            ListStates::select(app.selected_tab, &mut app.list_states).select_next();
        }

        KeyCode::Delete => {
            app.cmd_output_state.cmd_output.lines.clear();
        }

        KeyCode::Enter => {
            let sh = Shell::new()?;

            let command: Cmd = cmd!(sh, "stellar --help").timeout(Duration::from_secs(3));

            // Run the command with a timeout
            let res = command.read().unwrap_or_else(|e| "".parse().unwrap());

            app.cmd_output_state.cmd_output
               .push_line(Line::raw(ListStates::select(app.selected_tab, &mut app.list_states).selected().unwrap().to_string()));

            app.cmd_output_state.cmd_output.push_line(Line::raw(app.selected_tab.title().to_string()));

            app.cmd_output_state.cmd_output.push_line(Line::raw(res));
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

