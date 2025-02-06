use std::ptr::from_mut;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::Alignment;
use ratatui::text::{Line, ToLine, ToText};
use ratatui::widgets::block::title;
use ratatui::widgets::ScrollDirection;
use tokio::net::unix::uid_t;
use xshell::Cmd;
use StellarCliCmdName::{Env, ReadContractDataWasm, Version};
use crate::app::{App, AppResult, ListStates};
use crate::commands::commands::{CmdResponse, execute, StellarCliCmdName};
use crate::commands::commands::StellarCliCmdName::NetworkToggle;
use crate::ui::layout::CmdOutputScrollbar;

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
            ListStates::select_prev(app.selected_tab, &mut app.list_states);
        }
        KeyCode::Down => {
            ListStates::select_next(app.selected_tab, &mut app.list_states);
        }

        KeyCode::Tab => {
                app.cmd_output_state.cmd_output_scrollbar.scroll(ScrollDirection::Forward);
        }

        KeyCode::Delete => {
            app.cmd_output_state.cmd_output.lines.clear();
        }

        KeyCode::Enter => {
            let stellar_cli_cmd_name: StellarCliCmdName = NetworkToggle;
            // Run the command with a timeout
            let res: CmdResponse = execute(NetworkToggle);

            app.cmd_output_state.cmd_output.push_line(Line::raw(res.raw_cmd.to_string()));
            app.cmd_output_state.cmd_output.push_line(Line::raw(res.result));

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

            app.cmd_output_state
               .cmd_output
               .push_line(Line::raw(app.selected_tab.title().to_string()));
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
