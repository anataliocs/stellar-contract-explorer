use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Modifier;
use ratatui::style::Stylize;
use ratatui::widgets::{HighlightSpacing, List, ListItem, StatefulWidget, Tabs};
use strum::IntoEnumIterator;

use crate::app::{App, ListStates, SelectedTab};
use crate::app::SelectedTab::{Tab1, Tab2, Tab3, Tab4};

pub mod ui {}

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {

    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples

    let [tab_area, top_area, bot_area] = Layout::vertical(
        [Constraint::Max(3), Constraint::Max(4), Constraint::Fill(2)])
        .areas(frame.area());

    let titles = SelectedTab::iter().map(SelectedTab::title);
    let highlight_style = (Color::default(), app.selected_tab.palette().c700);
    let selected_tab_index = app.selected_tab as usize;


    frame.render_widget(
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding(" ", " ")
            .divider(" ")
            .block(Block::bordered()),
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
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .centered(),
        top_area,
    );

    frame.render_stateful_widget(
        match app.selected_tab {
            Tab1 => {
                List::new(
                    vec![ListItem::new("Item 1"),
                         ListItem::new("Item 2")]
                ).bg(Color::DarkGray)
                 .highlight_style(
                     Style::default()
                         .bg(Color::Yellow)
                         .add_modifier(Modifier::BOLD))
                 .highlight_symbol(">")
                 .highlight_spacing(HighlightSpacing::Always)
                 .block(Block::bordered().title("Extend TTL Scripts"))
            }
            Tab2 => {
                List::new(["item1", "item2"])
                    .highlight_style(Style::default())
                    .highlight_symbol(">")
                    .highlight_spacing(HighlightSpacing::Always)
                    .block(Block::bordered())
            }
            Tab3 => {
                List::new(["item1", "item2"])
                    .highlight_style(Style::default())
                    .highlight_symbol(">")
                    .highlight_spacing(HighlightSpacing::Always)
                    .block(Block::bordered())
            }
            Tab4 => {
                List::new(["item1", "item2"])
                    .highlight_style(Style::default())
                    .highlight_symbol(">")
                    .highlight_spacing(HighlightSpacing::Always)
                    .block(Block::bordered())
            }
        },
        bot_area, &mut ListStates::select(app.selected_tab, &mut app.list_states),
    );
}