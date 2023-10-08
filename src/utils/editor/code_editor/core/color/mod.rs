use std::fmt::Display;
use crossterm::style::{Stylize, StyledContent};

pub struct EditorColor;

impl EditorColor {
    #[inline]
    pub fn highlight_style<D>(content: D) -> StyledContent<D>
        where D: Display + Stylize<Styled = StyledContent<D>> {
        content.white().on_dark_red()
    }

    #[inline]
    pub fn line_active_style<D>(content: D) -> StyledContent<D>
        where D: Display + Stylize<Styled = StyledContent<D>> {
        content.bold().black().on_white()
    }
    #[inline]
    pub fn line_disabled_style<D>(content: D) -> StyledContent<D>
        where D: Display + Stylize<Styled = StyledContent<D>> {
        content.dark_grey().on_grey()
    }
}