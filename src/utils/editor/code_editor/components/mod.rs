mod core;
mod file_saver;
mod finder;
mod positioner;
mod replacer;

pub use self::core::Component;
pub use file_saver::FileSaver;
pub use finder::Finder;
pub use positioner::Positioner;
pub use replacer::Replacer;

use std::io;

use crossterm::event::KeyEvent;

use super::core::EditorState;

pub struct EditorComponentManager {
    pub is_in_component: bool,

    pub file_saver: FileSaver,
    pub positioner: Positioner,
    pub finder: Finder,
    pub replacer: Replacer,
}

impl EditorComponentManager {
    pub fn new() -> Self {
        Self {
            is_in_component: false,

            file_saver: FileSaver::new(),
            positioner: Positioner::new(),
            finder: Finder::new(),
            replacer: Replacer::new(),
        }
    }

    pub fn resolve(&mut self, current_state: EditorState, key: KeyEvent) -> io::Result<()> {
        match current_state {
            EditorState::Saving => self.file_saver.key_resolve(key)?,
            EditorState::Positioning => self.positioner.key_resolve(key)?,
            EditorState::Finding => self.finder.key_resolve(key)?,
            EditorState::Replacing => self.replacer.key_resolve(key)?,
            _ => unreachable!(),
        }
        return Ok(());
    }
}
