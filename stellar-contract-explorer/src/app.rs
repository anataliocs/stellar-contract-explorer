use std::collections::linked_list;
use std::error;
use std::fmt::{Debug, Formatter};
use std::ptr::{addr_of_mut, copy_nonoverlapping};

use futures::StreamExt;
use ratatui::buffer::Buffer;
use ratatui::layout::Alignment::Center;
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::style::palette::tailwind;
use ratatui::symbols;
use ratatui::text::Line;
use ratatui::widgets::{Block, Padding, Paragraph, Widget};
pub(crate) use ratatui::widgets::ListState;
use strum::{Display, EnumIter, FromRepr};

use crate::app::SelectedTab::{Tab1, Tab2, Tab3, Tab4};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;


pub struct ListStates {
    pub list_state: ListState,
    pub list_state2: ListState,
    pub list_state3: ListState,
    pub list_state4: ListState,
    pub func: Box<dyn Fn(SelectedTab, &mut Box<ListStates>) -> &mut ListState>,
}

impl ListStates {
    pub const fn select(selected_tab: SelectedTab, list_states: &mut Box<ListStates>) -> &mut ListState {
        match selected_tab {
            Tab1 => { &mut list_states.list_state }
            Tab2 => { &mut list_states.list_state2 }
            Tab3 => { &mut list_states.list_state3 }
            Tab4 => { &mut list_states.list_state4 }
        }
    }

    pub fn new(list_state: ListState, list_state2: ListState,
               list_state3: ListState, list_state4: ListState,
               func: Box<dyn Fn(SelectedTab, &mut Box<ListStates>) -> &mut ListState>) -> Self {
        Self { list_state, list_state2, list_state3, list_state4, func }
    }
}
/// Application.

pub struct App {
    /// Is the application running?
    pub running: bool,

    pub state: AppState,

    pub selected_tab: SelectedTab,

    pub list_states: Box<ListStates>,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, Debug)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Extend TTL")]
    Tab1,
    #[strum(to_string = "Restore Archived Data")]
    Tab2,
    #[strum(to_string = "Invoke Contract")]
    Tab3,
    #[strum(to_string = "Display Contract Info")]
    Tab4,
}


impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

impl SelectedTab {
    /// Return tab's name as a styled `Line`
    pub fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    /// A block surrounding the tab's content
    fn block(self) -> Block<'static> {
        Block::bordered()
            .title_alignment(Center)
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
            .style(Style::default())
    }

    pub const fn palette(self) -> tailwind::Palette {
        match self {
            Tab1 => tailwind::BLUE,
            Tab2 => tailwind::EMERALD,
            Tab3 => tailwind::INDIGO,
            Tab4 => tailwind::RED,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            state: AppState::Running,
            selected_tab: Tab1,
            list_states: Box::new(
                ListStates::new(ListState::default().with_offset(0).with_selected(Some(0)),
                                ListState::default().with_offset(0).with_selected(Some(0)),
                                ListState::default().with_offset(0).with_selected(Some(0)),
                                ListState::default().with_offset(0).with_selected(Some(0)),
                                Box::new(|selected_tab, mut list_states|
                                {
                                    match selected_tab {
                                        Tab1 => { &mut list_states.list_state }
                                        Tab2 => { &mut list_states.list_state2 }
                                        Tab3 => { &mut list_states.list_state3 }
                                        Tab4 => { &mut list_states.list_state4 }
                                    }
                                }))),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }
}
