use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::Alignment;
use ratatui::text::{Line, ToText};
use ratatui::widgets::block::title;
use StellarCliCmdName::{Env, ReadContractDataWasm, Version};
use crate::app::{App, AppResult, ListStates};
use crate::commands::commands::{execute, StellarCliCmdName};
use crate::commands::commands::StellarCliCmdName::NetworkToggle;

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
            let stellar_cli_cmd_name: StellarCliCmdName = NetworkToggle;
            // Run the command with a timeout
            let res: String = execute(NetworkToggle);

            match stellar_cli_cmd_name {
                Version => {}
                Env => {}
                ReadContractDataWasm => {}
                NetworkToggle => {
                    app.cmd_output_state.network_status.lines.remove(0);
                    app.cmd_output_state.network_status
                        .push_line(Line::raw("Network: Local"));
                    app.cmd_output_state.cmd_output
                        .push_line(Line::raw("Local Network Connected")
                            .alignment(Alignment::Left));
                }
            }

            app.cmd_output_state.cmd_output.push_line(Line::raw(
                ListStates::select(app.selected_tab, &mut app.list_states)
                    .selected()
                    .unwrap()
                    .to_string(),
            ));

            app.cmd_output_state
               .cmd_output
               .push_line(Line::raw(app.selected_tab.title().to_string()));

            app.cmd_output_state.cmd_output.push_line(Line::raw(res).alignment(Alignment::Left));
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
