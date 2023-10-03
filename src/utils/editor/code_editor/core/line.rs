use std::io;

use crossterm::style::Stylize;

use crate::utils::{
    editor::{direction::Direction, text_area::TextArea, tokenizer::TokenSequence},
    number_bit_count, Cursor, Terminal,
};

pub struct EditorLine {
    text_area: TextArea<TokenSequence>,
}

// state methods
impl EditorLine {
    #[inline]
    pub fn is_at_line_start(&self) -> io::Result<bool> {
        Ok(self.text_area.state_left()?.is_at_indent_start)
    }
    #[inline]
    pub fn is_at_line_end(&self) -> io::Result<bool> {
        Ok(self.text_area.state_right()?.is_at_content_end)
    }
    #[inline]
    pub fn is_at_after_indent(&self) -> io::Result<bool> {
        Ok(self.text_area.state_left()?.is_at_content_start)
    }

    #[inline]
    pub fn indent_count(&self) -> usize {
        self.text_area.indent_count()
    }
}

// editing methods
impl EditorLine {
    #[inline]
    pub fn move_cursor_to_start(&mut self, label_width: usize) -> io::Result<()> {
        self.update_label_width(label_width);
        self.text_area.move_cursor_to_start()
    }
    #[inline]
    pub fn move_cursor_after_indent(&mut self, label_width: usize) -> io::Result<()> {
        self.update_label_width(label_width);
        self.text_area.move_cursor_after_indent()
    }
    #[inline]
    pub fn move_cursor_to_end(&mut self, label_width: usize) -> io::Result<()> {
        self.update_label_width(label_width);
        self.text_area.move_cursor_to_end()
    }
    #[inline]
    pub fn move_cursor_horizontal(&mut self, dir: Direction) -> io::Result<()> {
        self.text_area.move_cursor_horizontal(dir)
    }

    #[inline]
    pub fn insert_char(&mut self, ch: char) -> io::Result<()> {
        self.text_area.insert_char(ch)
    }
    #[inline]
    pub fn delete_char(&mut self) -> io::Result<Option<char>> {
        self.text_area.delete_char()
    }

    #[inline]
    pub fn append_indent(&mut self) -> io::Result<()> {
        self.text_area.append_indent()
    }
    #[inline]
    pub fn remove_indent(&mut self) -> io::Result<()> {
        self.text_area.remove_indent()
    }
}

impl EditorLine {
    pub fn new(width: usize) -> Self {
        Self {
            text_area: TextArea::new(width, 1),
        }
    }

    pub fn render(&mut self, index: usize, label_width: usize) -> io::Result<()> {
        Cursor::move_to_col(0)?;
        Terminal::clear_after_cursor()?;

        // display label
        let index_width = number_bit_count(index);
        let space_width = label_width - index_width;
        let line_label_str =
            format!("{}{}", index.to_string().black(), " ".repeat(space_width)).on_white();
        print!("{}", line_label_str);

        self.text_area.margin_left = label_width;
        self.text_area.render()?;
        return Ok(());
    }

    pub fn find_all(&self, pat: &str) -> Option<Vec<usize>> {
        let mut text = self.content();
        let mut pos_offset = 0;
        let mut result_vec = vec![];
        while let Some(pos) = text.find(pat) {
            result_vec.push(pos + pos_offset);
            pos_offset += pos + pat.len();
            text = &text[(pos + pat.len())..];
        }

        if result_vec.is_empty() {
            return None;
        } else {
            return Some(result_vec);
        }
    }

    #[inline]
    fn update_label_width(&mut self, new_width: usize) {
        self.text_area.margin_left = new_width;
    }

    #[inline]
    pub fn init_indent(&mut self, indent: usize) {
        self.text_area.push_str(&" ".repeat(indent));
    }

    #[inline]
    pub fn cursor_pos(&self) -> io::Result<usize> {
        self.text_area.cursor_pos()
    }

    #[inline]
    pub fn content(&self) -> &str {
        self.text_area.content()
    }

    #[inline]
    pub fn push_str(&mut self, str: &str) {
        self.text_area.push_str(str);
    }

    #[inline]
    pub fn truncate(&mut self) -> io::Result<String> {
        self.text_area.truncate()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.text_area.len()
    }
}

#[test]
fn editorline_find_all_test() {
    let mut line = EditorLine::new(0);
    line.push_str("abc  abc  abc");

    assert_eq!(line.find_all("abc"), Some(vec![0, 5, 10]));
}
