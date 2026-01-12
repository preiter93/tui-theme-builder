use ratatui::{
    style::{Color, Style},
    widgets::BorderType,
};
use serde::Deserialize;
use tui_theme_builder::ThemeBuilder;

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub primary: Color,
}

#[derive(Debug, ThemeBuilder, PartialEq, Eq)]
#[builder(context=Colors)]
pub struct Theme {
    /// Annotate styles with 'fg=color', 'bg=color' or add modifiers,
    /// e.g. 'bold' or 'underlined'.
    #[style(fg=primary, bg=primary, bold, underlined)]
    pub base: Style,
    /// Annotate border types with 'value=field' to reference a context field,
    /// or use a variant name directly, e.g. 'plain', 'rounded', 'thick', etc.
    /// Note: If accessed with 'value=field' the 'field' must start with a
    /// capital letter.
    #[border_type(thick)]
    pub border: BorderType,
}

impl Default for Colors {
    fn default() -> Self {
        let s = r##"
        "primary" = "#000000"
        "##;
        toml::from_str(s).unwrap()
    }
}

fn main() {
    let colors = Colors::default();
    let theme = Theme::build(&colors);
    println!("{theme:#?}");

    assert_eq!(
        theme,
        Theme {
            base: Style::default()
                .fg(ratatui::style::Color::Rgb(0, 0, 0))
                .bg(ratatui::style::Color::Rgb(0, 0, 0))
                .bold()
                .underlined(),
        }
    );
}
