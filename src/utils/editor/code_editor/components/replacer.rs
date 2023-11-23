use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::utils::{
    editor::{code_editor::cursor_pos::EditorCursorPos, text_area::TextArea},
    LoopTraverser,
};

use super::{
    core::{ComponentController, ComponentHistory},
    Component,
};

#[derive(PartialEq)]
enum ReplacerState {
    Searching,
    Replacing,
}

pub struct Replacer {
    state: ReplacerState,
    match_list: LoopTraverser<EditorCursorPos>,

    search_history: ComponentHistory,
    replace_history: ComponentHistory,

    searcher: ComponentController,
    replacer: ComponentController,
}

impl Replacer {
    const REPLACE_PROMPT: &'static str = "Replace: ";
    const REPLACE_BUTTON: &'static str = "[Ctrl + S / N / A]";

    pub fn new() -> Self {
        let mut searcher_controller = Self::init_controller();
        let mut replacer_controller = ComponentController {
            prompt: Self::REPLACE_PROMPT,
            button: Self::REPLACE_BUTTON,
            text_area: TextArea::new(Self::REPLACE_PROMPT.len(), Self::REPLACE_BUTTON.len()),
            position: -1,
            editable: true,
        };
        searcher_controller.text_area.set_placeholder(ComponentHistory::HISTORY_PLACEHOLDER);
        replacer_controller.text_area.set_placeholder(ComponentHistory::HISTORY_PLACEHOLDER);

        return Self {
            state: ReplacerState::Searching,
            match_list: LoopTraverser::new(false),

            search_history: ComponentHistory::new(),
            replace_history: ComponentHistory::new(),

            searcher: searcher_controller,
            replacer: replacer_controller,
        };
    }

    #[inline]
    pub fn first(&self) -> Option<&EditorCursorPos> {
        self.match_list.first()
    }

    #[inline]
    pub fn current(&self) -> Option<&EditorCursorPos> {
        self.match_list.current()
    }

    #[inline]
    pub fn next(&mut self) -> Option<&EditorCursorPos> {
        self.match_list.next()
    }

    #[inline]
    pub fn search_text(&self) -> &str {
        self.searcher.text_area.content()
    }
    #[inline]
    pub fn replace_text(&self) -> &str {
        self.replacer.text_area.content()
    }

    // when pressed `search_key` (Enter) and exists search result,
    // this handler will be called.
    pub fn search_handler(&mut self, pos_list: Vec<EditorCursorPos>) -> io::Result<()> {
        self.search_history.append(self.search_text().to_owned());
        self.match_list.set_content(pos_list);
        self.replacer.open()?;
        self.state = ReplacerState::Replacing;
        return Ok(());
    }

    // when pressed `replace_one_key` or `replace_all_key`,
    // this handler will be called.
    pub fn replace_handler(&mut self) {
        let current_content = self.replace_text();
        if let Some(last_content) = self.replace_history.last() {
            // avoid repetitive content
            if current_content == last_content {
                return;
            }
        }
        self.replace_history.append(current_content.to_owned());
    }

    pub fn reset(&mut self) {
        self.state = ReplacerState::Searching;
        self.searcher.text_area.clear();
        self.replacer.text_area.clear();
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
        if !(key.modifiers == KeyModifiers::NONE || key.modifiers == KeyModifiers::SHIFT) {
            return Ok(());
        }

        match key.code {
            KeyCode::Up | KeyCode::Down => {
                let (current_content, target_history) = match self.state {
                    ReplacerState::Searching => {
                        (self.search_text().to_owned(), &mut self.search_history)
                    }
                    ReplacerState::Replacing => {
                        (self.replace_text().to_owned(), &mut self.replace_history)
                    }
                };
                let history_content = match key.code {
                    KeyCode::Up => {
                        if !target_history.use_history {
                            target_history.set_cached(current_content);
                        }
                        target_history.previous()
                    }
                    KeyCode::Down => target_history.next(),
                    _ => unreachable!(),
                };
                if let Some(str) = history_content {
                    let text_area = match self.state {
                        ReplacerState::Searching => &mut self.searcher.text_area,
                        ReplacerState::Replacing => &mut self.replacer.text_area,
                    };
                    text_area.set_content(str);
                    text_area.move_cursor_to_end(false)?;
                    text_area.render()?;
                }
            }
            k if TextArea::is_editing_key(k) => match self.state {
                ReplacerState::Searching => &mut self.searcher,
                ReplacerState::Replacing => &mut self.replacer,
            }
            .edit(key.code)?,
            _ => {}
        }
        return Ok(());
    }
}
