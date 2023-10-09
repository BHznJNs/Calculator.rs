use std::io;

use crossterm::style::Stylize;

use super::super::cursor_pos::{EditorCursorPos, TerminalCursorPos};
use super::EditorState;
use crate::utils::editor::code_editor::core::color::EditorColor;
use crate::utils::{Cursor, Terminal};

pub struct EditorDashboard {
    cursor_pos: EditorCursorPos,
    state: EditorState,

    // this cursor position is used to temporarily
    // save and restore cursor.
    temp_cursor_pos: TerminalCursorPos,

    // this state is used to cache current state when
    // component state is set.
    saved_state: EditorState,
}

impl EditorDashboard {
    pub fn new() -> Self {
        Self {
            cursor_pos: EditorCursorPos { row: 1, col: 1 },
            state: EditorState::Saved,

            temp_cursor_pos: TerminalCursorPos { row: 1, col: 1 },
            saved_state: EditorState::Saved,
        }
    }

    pub fn render(&mut self) -> io::Result<()> {
        self.temp_cursor_pos.save_pos()?;

        // move cursor to start of the last row
        Cursor::move_to_row(Terminal::height() - 1)?;
        Cursor::move_to_col(0)?;

        let state_str = format!(" {} ", self.state);
        let cursor_pos_str = format!(" {} ", self.cursor_pos);

        // `2` here is space for left-margin and right-margin
        let remain_space = Terminal::width() - state_str.len() - cursor_pos_str.len();
        let divider_str = " ".repeat(remain_space).on_white();

        print!(
            "{}{divider_str}{}",
            EditorColor::highlight_style(state_str),
            EditorColor::highlight_style(cursor_pos_str)
        );
        self.temp_cursor_pos.restore_pos()?;
        return Ok(());
    }

    #[inline]
    pub fn state(&self) -> EditorState {
        self.state
    }

    pub fn set_state(&mut self, new_state: EditorState) -> io::Result<()> {
        if new_state.is_component_state() {
            // cache current state
            self.saved_state = self.state;
        } else {
            self.saved_state = new_state;
        }
        self.state = new_state;

        self.render()?;
        return Ok(());
    }

    pub fn restore_state(&mut self) -> io::Result<()> {
        self.state = self.saved_state;
        self.render()?;
        return Ok(());
    }

    pub fn set_cursor_pos(&mut self, pos: EditorCursorPos) -> io::Result<()> {
        (self.cursor_pos.row, self.cursor_pos.col) = (pos.row, pos.col);
        self.render()?;
        return Ok(());
    }
}
