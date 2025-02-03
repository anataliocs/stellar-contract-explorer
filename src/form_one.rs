use promkit::crossterm::style::Color;
use promkit::preset::form::Form;
use promkit::style::StyleBuilder;
use promkit::text_editor;
use text_editor::State;

pub fn form_factory() -> Form {
    Form::new([
        get_editor_state("❯❯ ", Color::DarkRed, Color::DarkCyan),
        get_editor_state("❯❯ ", Color::DarkRed, Color::DarkCyan),
        get_editor_state("❯❯ ", Color::DarkRed, Color::DarkCyan),
    ])
}

fn get_editor_state(prompt: &str, prefix_color: Color, inactive_color: Color) -> State {
    State {
        texteditor: Default::default(),
        history: Default::default(),
        prefix: String::from(prompt),
        mask: Default::default(),
        prefix_style: StyleBuilder::new().fgc(prefix_color).build(),
        active_char_style: StyleBuilder::new().bgc(inactive_color).build(),
        inactive_char_style: StyleBuilder::new().build(),
        edit_mode: Default::default(),
        word_break_chars: Default::default(),
        lines: Default::default(),
    }
    .clone()
}
