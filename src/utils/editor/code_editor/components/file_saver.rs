use std::{
    fs::{self, File},
    io,
    path::Path,
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::core::{Component, ComponentController};

pub struct FileSaver {
    editor_content: String,
    comp: ComponentController,
}

impl FileSaver {
    const DEFAULT_FILE_NAME: &str = "temp.txt";

    pub fn new() -> Self {
        let mut controller = Self::init_controller();
        controller.text_area.set_content(Self::DEFAULT_FILE_NAME);
        return Self {
            editor_content: String::new(),
            comp: controller,
        };
    }

    fn save(&self) -> io::Result<()> {
        let target_path_str = self.comp.text_area.content();
        let target_path = Path::new(target_path_str);

        if !target_path.exists() {
            File::create(target_path)?;
        }
        let bytes_to_write = self.editor_content.as_bytes();
        fs::write(target_path_str, bytes_to_write)?;
        return Ok(());
    }

    #[inline]
    pub fn is_save_callback_key(key: KeyEvent) -> bool {
        key.modifiers == KeyModifiers::NONE && key.code == KeyCode::Enter
    }

    #[inline]
    pub fn set_content(&mut self, content: String) {
        self.editor_content = content;
    }

    #[inline]
    pub fn set_path(&mut self, path: &str) {
        self.comp.text_area.set_content(path);
    }
}

impl Component for FileSaver {
    const PROMPT: &'static str = "Path: ";
    const BUTTON: &'static str = "[Enter]";
    const POSITION: isize = 1;
    const EDITABLE: bool = true;

    #[inline]
    fn open(&mut self) -> io::Result<()> {
        self.comp.open()
    }

    fn key_resolve(&mut self, key: KeyEvent) -> io::Result<()> {
        if key.modifiers == KeyModifiers::NONE || key.modifiers == KeyModifiers::SHIFT {
            match key.code {
                KeyCode::Enter => self.save()?,
                k if ComponentController::is_editing_key(k) => self.comp.edit(k)?,
                _ => {}
            }
        }
        return Ok(());
    }
}
