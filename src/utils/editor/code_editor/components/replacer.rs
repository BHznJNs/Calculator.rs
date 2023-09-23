use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::utils::{
    editor::code_editor::{cursor_pos::EditorCursorPos, text_area::TextArea},
    LoopTraverser,
};

use super::{core::ComponentController, Component};

#[derive(PartialEq)]
enum ReplacerState {
    Searching,
    Replacing,
}

pub struct Replacer {
    state: ReplacerState,
    match_list: LoopTraverser<EditorCursorPos>,

    searcher: ComponentController,
    replacer: ComponentController,
}

impl Replacer {
    const REPLACE_PROMPT: &'static str = "Replace: ";
    const REPLACE_BUTTON: &'static str = "[Ctrl + S / N / A]";

    pub fn new() -> Self {
        Self {
            state: ReplacerState::Searching,
            match_list: LoopTraverser::new(false),

            searcher: Self::init_controller(),
            replacer: ComponentController {
                prompt: Self::REPLACE_PROMPT,
                button: Self::REPLACE_BUTTON,
                text_area: TextArea::new(Self::REPLACE_PROMPT.len(), Self::REPLACE_BUTTON.len()),
                position: -1,
                editable: true,
            },
        }
    }

    #[inline]
    pub fn first<'a>(&'a self) -> Option<&'a EditorCursorPos> {
        self.match_list.first()
    }

    #[inline]
    pub fn current<'a>(&'a self) -> &'a EditorCursorPos {
        self.match_list.current()
    }

    #[inline]
    pub fn next<'a>(&'a mut self) -> Option<&'a EditorCursorPos> {
        self.match_list.next()
    }

    // when pressed `search_key` (Enter) and exists search result,
    // this handler will be called.
    pub fn search_handler(&mut self, pos_list: Vec<EditorCursorPos>) -> io::Result<()> {
        self.match_list.set_content(pos_list);
        self.replacer.open()?;
        self.state = ReplacerState::Replacing;
        return Ok(());
    }

    #[inline]
    pub fn search_text<'a>(&'a self) -> &'a str {
        self.searcher.text_area.content()
    }
    #[inline]
    pub fn replace_text<'a>(&'a self) -> &'a str {
        self.replacer.text_area.content()
    }

    #[inline]
    pub fn reset(&mut self) {
        self.state = ReplacerState::Searching;
        self.match_list.clear();
    }

    // --- --- --- --- --- ---

    #[inline]
    pub fn is_search_key(&self, key: KeyEvent) -> bool {
        self.state == ReplacerState::Searching
            && key.modifiers == KeyModifiers::NONE
            && key.code == KeyCode::Enter
    }
    #[inline]
    pub fn is_next_key(&self, key: KeyEvent) -> bool {
        self.state == ReplacerState::Replacing
            && key.modifiers == KeyModifiers::CONTROL
            && key.code == KeyCode::Char('n')
    }
    #[inline]
    pub fn is_replace_one_key(&self, key: KeyEvent) -> bool {
        self.state == ReplacerState::Replacing
            && key.modifiers == KeyModifiers::CONTROL
            && key.code == KeyCode::Char('s')
    }
    #[inline]
    pub fn is_replace_all_key(&self, key: KeyEvent) -> bool {
        self.state == ReplacerState::Replacing
            && key.modifiers == KeyModifiers::CONTROL
            && key.code == KeyCode::Char('a')
    }
}

impl Component for Replacer {
    const BUTTON: &'static str = "[Enter]";
    const PROMPT: &'static str = "Search: ";
    const EDITABLE: bool = true;
    const POSITION: isize = -1;

    fn open(&mut self) -> io::Result<()> {
        match self.state {
            ReplacerState::Searching => &mut self.searcher,
            ReplacerState::Replacing => &mut self.replacer,
        }
        .open()
    }
    fn key_resolve(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.modifiers {
            KeyModifiers::NONE | KeyModifiers::SHIFT
                if ComponentController::is_editing_key(key.code) =>
            {
                match self.state {
                    ReplacerState::Searching => &mut self.searcher,
                    ReplacerState::Replacing => &mut self.replacer,
                }
                .edit(key.code)?
            }
            _ => {}
        }
        return Ok(());
    }
}
