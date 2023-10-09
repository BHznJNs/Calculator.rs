mod accent;

use std::fmt::Display;

use crossterm::style::{StyledContent, Stylize};

use accent::AccentColor;

pub struct EditorColor;
static mut ACCENT_COLOR: AccentColor = AccentColor::Red;

impl EditorColor {
    #[inline]
    pub fn set_accent_color(color: &str) {
        unsafe { ACCENT_COLOR = AccentColor::from(color) }
    }

    pub fn highlight_style<D>(content: D) -> StyledContent<D>
    where
        D: Display + Stylize<Styled = StyledContent<D>>,
    {
        let mut styled = content.white();
        styled = match unsafe { &ACCENT_COLOR } {
            AccentColor::Red => styled.on_red(),
            AccentColor::Blue => styled.on_blue(),
            AccentColor::DarkRed => styled.on_dark_red(),
            AccentColor::DarkBlue => styled.on_dark_blue(),
            AccentColor::DarkGrey => styled.on_dark_grey(),
            AccentColor::DarkCyan => styled.on_dark_cyan(),
            AccentColor::DarkYellow => styled.on_dark_yellow(),
            AccentColor::DarkMagenta => styled.on_magenta(),
        };
        return styled;
    }

    #[inline]
    pub fn line_active_style<D>(content: D) -> StyledContent<D>
    where
        D: Display + Stylize<Styled = StyledContent<D>>,
    {
        content.bold().black().on_white()
    }
    #[inline]
    pub fn line_disabled_style<D>(content: D) -> StyledContent<D>
    where
        D: Display + Stylize<Styled = StyledContent<D>>,
    {
        content.dark_grey().on_grey()
    }
}
