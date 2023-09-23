use std::io;

use crossterm::style::Stylize;

use crate::utils::{Cursor, Terminal};

pub struct EditorInit;

impl EditorInit {
    pub fn display_title() {
        let term_width = Terminal::width();
        let title_str = format!("Rusditor v{}", env!("CARGO_PKG_VERSION"));
        let esc_button_str = " [Esc] ";

        let elements_width = title_str.len() + esc_button_str.len();
        let padding_width1 = (term_width - elements_width) / 2;
        let padding_width2 = term_width - padding_width1 - elements_width;
        let (padding_str1, padding_str2) = (" ".repeat(padding_width1), " ".repeat(padding_width2));

        print!(
            "{}{}{}{}",
            padding_str1.on_white(),
            title_str.bold().black().on_white(),
            padding_str2.on_white(),
            esc_button_str.white().on_dark_red(),
        );
    }

    pub fn display_border() -> io::Result<()> {
        // print left and right border
        for _ in 1..Terminal::height() {
            print!("{}", "  ".on_white());
            Cursor::down(1)?;
            Cursor::move_to_col(0)?;
        }
        Terminal::flush()?;
        return Ok(());
    }
}
