use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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
            ListStates::select(app.selected_tab, &mut app.list_states).select_next();
        }
        KeyCode::Down => {
            ListStates::select(app.selected_tab, &mut app.list_states).select_previous();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

