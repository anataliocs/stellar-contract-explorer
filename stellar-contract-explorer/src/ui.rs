use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Modifier, Text};
use ratatui::style::Stylize;
use ratatui::widgets::{HighlightSpacing, List, ListItem, Tabs};
use strum::IntoEnumIterator;

use crate::app::{App, ListStates, SelectedTab};
use crate::app::SelectedTab::{Tab1, Tab2, Tab3, Tab4};

/// Renders the user interface widgets.
pub fn render<'a>(app: &'a mut App, frame: &mut Frame) {

    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples

    let [tab_area, top_area, bot_area] = Layout::vertical(
        [Constraint::Max(3), Constraint::Max(4), Constraint::Fill(2)])
        .areas(frame.area());


    let [bot_left, bot_right] = Layout::horizontal([
        Constraint::Fill(2),
        Constraint::Fill(4)])
        .areas(bot_area);

    let titles = SelectedTab::iter().map(SelectedTab::title);
    let highlight_style = (Color::default(), app.selected_tab.palette().c950);
    let selected_tab_index = app.selected_tab as usize;

    frame.render_widget(
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding(" ", " ")
            .divider(" ")
            .block(Block::bordered().border_type(BorderType::Rounded)),
        tab_area,
    );

    frame.render_widget(
        Paragraph::new(
            "Press `Esc`, `Ctrl-C` or `q` to quit.\n\
                Press left and right to move between tabs.\n")
            .block(
                Block::bordered()
                    .title("Stellar Contract Explorer")
                    .title_alignment(Alignment::Center)
                    .title_style(Style::default().add_modifier(Modifier::BOLD))
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow).bg(Color::Black))
            .centered(),
        top_area,
    );

    frame.render_stateful_widget(
        match app.selected_tab {
            Tab1 => {
                list_factory(vec![ListItem::new("Extend Instance TTL"),
                                  ListItem::new("Extend Persistence TTL"),
                                  ListItem::new("Extend Temporary TTL"),
                                  ListItem::new("Generate Data Key"),
                ], "Extend TTL Scripts")
            }
            Tab2 => {
                list_factory(vec![ListItem::new("Restore Persistent Storage"),
                                  ListItem::new("Restore Contract Instance"),
                                  ListItem::new("Restore Contract Code Hash"),
                                  ListItem::new("Restore Instance Storage"),
                ], "Restore Archived Data Scripts")
            }
            Tab3 => {
                list_factory(vec![ListItem::new("Set Persistent Data"),
                                  ListItem::new("Set Instance Data"),
                                  ListItem::new("Extend Persistent TTL"),
                                  ListItem::new("Extend Instance TTL"),
                ], "Contract Invocation Scripts")
            }
            Tab4 => {
                list_factory(vec![ListItem::new("Show Contract Data"),
                                  ListItem::new("Show Invocations"),
                                  ListItem::new("Show Storage TTLs"),
                                  ListItem::new("Show Misc data"),
                ], "Display Contract Info Scripts")
            }
        },
        bot_left, &mut ListStates::select(app.selected_tab, &mut app.list_states),
    );


    frame.render_widget(Paragraph::new(Text::to_owned(&app.cmd_output_state.cmd_output).clone())
                            .block(
                                Block::bordered()
                                    .title("Command Output")
                                    .title_alignment(Alignment::Center)
                                    .title_style(Style::default().add_modifier(Modifier::BOLD))
                                    .border_type(BorderType::Rounded),
                            )
                            .style(Style::default().fg(Color::Yellow).bg(Color::Black))
                            .centered(), bot_right);
}

fn list_factory<'a>(list_items: Vec<ListItem<'a>>, title: &'a str) -> List<'a> {
    List::new(
        list_items
    ).bg(Color::Black)
     .highlight_style(
         Style::default()
             .bg(Color::Yellow)
             .add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED))
     .highlight_symbol(" ❯❯ ")
     .highlight_spacing(HighlightSpacing::Always)
     .block(Block::bordered()
         .title(title)
         .title_style(Style::default().add_modifier(Modifier::BOLD)))
}

