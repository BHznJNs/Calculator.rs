mod color;
mod style;

use std::fmt::Display;

use {color::Color, style::Style};

pub struct ColoredString {
    pub ft_color : Color,
    // may use default bg-color
    pub bg_color : Option<Color>,
    pub style    : Style,
}

impl ColoredString {
    pub fn output<T: Display>(&self, content: T) -> String {
        let ft_color_str = self.ft_color.get_fg_str();
        let style_str = self.style.get_str();

        let bg_color_str =
        match self.bg_color {
            Some(bg) => format!("\x1b[{}m", bg.get_bg_str()),
            None => "".to_owned(),
        };

        format!("\x1b[{style_str};{ft_color_str}m{bg_color_str}{content}\x1b[0m")
    }
}

// --- --- --- --- --- ---

pub const BOOLEAN_COLORED: ColoredString = ColoredString {
    ft_color : Color::Yellow,
    bg_color : None,
    style    : Style::Normal,
};
pub const NUMBER_COLORED: ColoredString = ColoredString {
    ft_color : Color::BrightYellow,
    bg_color : None,
    style    : Style::Normal,
};
pub const STRING_COLORED: ColoredString = ColoredString {
    ft_color : Color::BrightGreen,
    bg_color : None,
    style    : Style::Normal,
};
pub const INTERNAL_COLORED: ColoredString = ColoredString {
    ft_color : Color::Cyan,
    bg_color : None,
    style    : Style::Normal,
};

pub const ERROR_COLORED: ColoredString = ColoredString {
    ft_color : Color::BrightWhite,
    bg_color : Some(Color::BrightRed),
    style    : Style::Bold,
};