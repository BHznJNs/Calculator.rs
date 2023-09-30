use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::utils::editor::{code_editor::cursor_pos::EditorCursorPos, text_area::TextArea};

use super::{core::ComponentController, Component};

pub struct Positioner {
    target: EditorCursorPos,
    comp: ComponentController,
}

impl Positioner {
    pub fn new() -> Self {
        let initial_cursor_pos = EditorCursorPos { row: 1, col: 1 };
        let mut controller = Self::init_controller();
        controller
            .text_area
            .set_placeholder(&initial_cursor_pos.short_display());
        return Self {
            target: initial_cursor_pos,
            comp: controller,
        };
    }

    #[inline]
    pub fn set_cursor_pos(&mut self, pos: EditorCursorPos) {
        let pos_str = pos.short_display();
        self.comp.text_area.set_placeholder(&pos_str);
        self.target = pos;
    }

    #[inline]
    pub fn is_positioning_key(key: KeyEvent) -> bool {
        key.modifiers == KeyModifiers::NONE && key.code == KeyCode::Enter
    }

    #[inline]
    pub fn get_target(&self) -> EditorCursorPos {
        self.target.clone()
    }
}

impl Component for Positioner {
    const PROMPT: &'static str = "Jump to: ";
    const BUTTON: &'static str = "[Enter]";
    const POSITION: isize = -1;
    const EDITABLE: bool = true;

    #[inline]
    fn open(&mut self) -> io::Result<()> {
        self.comp.open()
    }

    fn key_resolve(&mut self, key: KeyEvent) -> io::Result<()> {
        if !(key.modifiers == KeyModifiers::NONE || key.modifiers == KeyModifiers::SHIFT) {
            return Ok(());
        }

        match key.code {
            KeyCode::Enter => {
                let target_pos_str = self.comp.text_area.content();
                let parsed_pos = EditorCursorPos::parse(target_pos_str);
                self.comp.text_area.clear();

                if let Some(pos) = parsed_pos {
                    self.target = pos
                }
            }
            k if TextArea::is_editing_key(k) => self.comp.edit(k)?,
            _ => {}
        }
        return Ok(());
    }
}
