mod content;

use std::io;

use crossterm::{event::KeyCode, style::Stylize};

use crate::{
    public::env::ENV,
    utils::{Cursor, Terminal},
};

use super::direction::Direction;

pub use content::TextAreaContent;

pub(super) struct Placeholder {
    content: String,
}

impl TextAreaContent for Placeholder {
    fn new() -> Self {
        Placeholder {
            content: String::new(),
        }
    }
    #[inline]
    fn get(&self) -> &String {
        &self.content
    }
    #[inline]
    fn get_mut(&mut self) -> &mut String {
        &mut self.content
    }
    #[inline]
    fn change_handler(&mut self) {}
}

pub struct TextArea<C: TextAreaContent> {
    content: C,
    placeholder: Placeholder,

    pub margin_left: usize,
    pub margin_right: usize,

    overflow_left: usize,
    overflow_right: usize,
}

#[derive(Debug)]
pub struct TextAreaStateLeft {
    // cursor is at left side of text_area,
    // but may not at content start.
    pub is_at_left_side: bool,
    // cursor is at left side of indent.
    pub is_at_indent_start: bool,
    // cursor is at right side of indent and
    // start of actual content (content without indent).
    pub is_at_content_start: bool,
}

#[derive(Debug)]
pub struct TextAreaStateRight {
    // cursor is at right side of text_area,
    // but may not at content end.
    pub is_at_right_side: bool,
    // cursor is at content end.
    pub is_at_content_end: bool,
}

impl TextArea<String> {
    // this static method is used when TextArea uses
    // `String` and `TokenSequence` as content,
    // use `String` here is just for easier to call this method
    // in external scope.
    pub fn is_editing_key(key: KeyCode) -> bool {
        matches!(
            key,
            KeyCode::Backspace | KeyCode::Left | KeyCode::Right | KeyCode::Char(_)
        )
    }
}

// state calculating methods
impl<C: TextAreaContent> TextArea<C> {
    // returns number of continuous alphabetic char.
    // e.g.
    //   in : ['a', 'b', ' ', 'c']
    //   out: 2
    // --- --- --- --- --- ---
    //   in : [' ', 'a', 'b']
    //   out: 1
    fn continuous_word_count(chars: impl Iterator<Item = char>) -> usize {
        let counter = chars
            .map_while(|ch| ch.is_alphabetic().then_some(()))
            .count();
        return counter;
    }

    #[inline]
    pub fn is_at_left_side(&self) -> io::Result<bool> {
        Ok(Cursor::pos_col()? == self.margin_left)
    }
    #[inline]
    pub fn is_at_right_side(&self) -> io::Result<bool> {
        Ok(Cursor::pos_col()? == Terminal::width() - 1)
    }

    pub fn state_left(&self) -> io::Result<TextAreaStateLeft> {
        let is_at_left_side = self.is_at_left_side()?;
        let is_at_indent_start = is_at_left_side && self.overflow_left == 0;
        let is_at_content_start = self.cursor_pos()? == self.indent_count();
        return Ok(TextAreaStateLeft {
            is_at_left_side,
            is_at_indent_start,
            is_at_content_start,
        });
    }
    pub fn state_right(&self) -> io::Result<TextAreaStateRight> {
        let cursor_pos_col = Cursor::pos_col()?;
        let is_at_right_side = self.is_at_right_side()?;
        // let is_at_content_end = self.cursor_pos()? == self.len();
        let is_at_content_end = cursor_pos_col == (self.len() + self.margin_left)
            || cursor_pos_col == (self.len() - self.overflow_left + self.margin_left);
        return Ok(TextAreaStateRight {
            is_at_right_side,
            is_at_content_end,
        });
    }

    // returns the space count at the start of content
    pub fn indent_count(&self) -> usize {
        let mut result = 0;
        for ch in self.content().chars() {
            if ch == ' ' {
                result += 1;
            } else {
                break;
            }
        }
        return result;
    }
}

// editing methods
impl<C: TextAreaContent> TextArea<C> {
    pub fn visible_area_width(&self) -> usize {
        let term_width = Terminal::width();
        return term_width - self.margin_left - self.margin_right;
    }

    fn overflow_refresh(&mut self) {
        let visible_area_width = self.visible_area_width();
        if self.len() > visible_area_width {
            self.overflow_left = self.len() - visible_area_width - self.overflow_right;
        } else {
            self.overflow_left = 0;
            self.overflow_right = 0;
        }
    }

    pub fn move_cursor_after_indent(&mut self) -> io::Result<()> {
        self.move_cursor_to_start()?;
        for _ in 0..self.indent_count() {
            self.move_cursor_horizontal(Direction::Right)?;
        }
        return Ok(());
    }
    pub fn move_cursor_to_start(&mut self) -> io::Result<()> {
        if self.len() >= self.visible_area_width() {
            self.overflow_right += self.overflow_left;
            self.overflow_left = 0;
            self.render()?;
        }
        Cursor::move_to_col(self.margin_left)?;
        return Ok(());
    }
    pub fn move_cursor_to_end(&mut self) -> io::Result<()> {
        if self.len() >= self.visible_area_width() {
            Cursor::move_to_col(Terminal::width() - 1)?;
            self.overflow_left += self.overflow_right;
            self.overflow_right = 0;
            self.render()?;
        } else {
            let line_end_pos = self.margin_left + self.len();
            Cursor::move_to_col(line_end_pos)?;
        }
        return Ok(());
    }

    pub fn move_cursor_horizontal(&mut self, dir: Direction) -> io::Result<()> {
        // log(format!("state_left: {:#?}, state_right: {:#?}", self.state_left()?, self.state_right()?))?;

        match dir {
            Direction::Left => {
                let state = self.state_left()?;
                if state.is_at_indent_start {
                    return Ok(());
                }

                if state.is_at_left_side {
                    self.overflow_left -= 1;
                    self.overflow_right += 1;
                } else {
                    Cursor::left(1)?;
                    return Ok(()); // skip rerender
                }
            }
            Direction::Right => {
                let state = self.state_right()?;
                if state.is_at_content_end {
                    return Ok(());
                }

                if state.is_at_right_side {
                    self.overflow_right -= 1;
                    self.overflow_left += 1;
                } else {
                    Cursor::right(1)?;
                    return Ok(()); // skip rerender
                }
            }
            _ => unreachable!(),
        }
        self.render()?;
        return Ok(());
    }

    pub fn jump_to_word_edge(&mut self, dir: Direction) -> io::Result<()> {
        let cursor_pos = self.cursor_pos()?;
        let mut displacement = match dir {
            Direction::Left => {
                let iter = self.content()[..cursor_pos].chars().rev();
                Self::continuous_word_count(iter)
            }
            Direction::Right => {
                let iter = self.content()[cursor_pos..].chars();
                Self::continuous_word_count(iter)
            }
            _ => unreachable!(),
        };

        // when displacement is 0 and cursor is not at left and right end
        if displacement == 0
            && !(dir == Direction::Left && self.state_left()?.is_at_indent_start)
            && !(dir == Direction::Right && self.state_right()?.is_at_content_end)
        {
            displacement = 1;
        }

        for _ in 0..displacement {
            self.move_cursor_horizontal(dir)?;
        }
        return Ok(());
    }
}

impl<C: TextAreaContent> TextArea<C> {
    pub fn new(margin_left: usize, margin_right: usize) -> Self {
        Self {
            content: C::new(),
            placeholder: Placeholder::new(),

            overflow_left: 0,
            overflow_right: 0,

            margin_left,
            margin_right,
        }
    }

    pub fn render(&self) -> io::Result<()> {
        let visible_area_width = self.visible_area_width();
        let rendered_content = if self.len() == 0 && !self.placeholder.is_empty() {
            self.placeholder
                .rendered_content(0, visible_area_width)
                .dim()
        } else {
            self.content
                .rendered_content(self.overflow_left, visible_area_width)
                .stylize()
        };

        let saved_cursor_pos = Cursor::pos_col()?;
        Cursor::move_to_col(self.margin_left)?;
        print!("{}", rendered_content);
        Cursor::move_to_col(saved_cursor_pos)?;
        return Ok(());
    }

    pub fn insert_char(&mut self, ch: char) -> io::Result<()> {
        let insert_pos = self.cursor_pos()?;
        self.content.insert(insert_pos, ch);

        if self.content.len() > self.visible_area_width() {
            self.overflow_left += 1;
        } else {
            Cursor::right(1)?;
        }
        self.render()?;
        return Ok(());
    }

    pub fn delete_char(&mut self) -> io::Result<Option<char>> {
        if self.state_left()?.is_at_indent_start {
            return Ok(None);
        }

        let remove_pos = self.cursor_pos()? - 1;
        let removed_ch = self.content.remove(remove_pos);

        if self.content.len() >= self.visible_area_width() {
            self.overflow_left -= 1;
        } else {
            Cursor::left(1)?;
        }
        self.render()?;
        return Ok(Some(removed_ch));
    }

    pub fn append_indent(&mut self) -> io::Result<()> {
        let current_indent = self.indent_count();
        let default_indent_size = unsafe { ENV.options.indent_size };
        let indent_size_to_append = default_indent_size - current_indent % default_indent_size;

        for _ in 0..indent_size_to_append {
            self.content.insert(0, ' ');
            self.move_cursor_horizontal(Direction::Right)?;
        }
        self.render()?;
        return Ok(());
    }
    pub fn remove_indent(&mut self) -> io::Result<()> {
        let current_indent = self.indent_count();
        if current_indent == 0 {
            // if no indent, directly return.
            return Ok(());
        }

        let default_indent_size = unsafe { ENV.options.indent_size };
        let mut indent_size_to_remove = current_indent % default_indent_size;
        if indent_size_to_remove == 0 {
            indent_size_to_remove += default_indent_size;
        }

        for _ in 0..indent_size_to_remove {
            self.content.remove(0);
            self.move_cursor_horizontal(Direction::Left)?;
        }
        self.render()?;
        return Ok(());
    }

    #[inline]
    pub fn push_str(&mut self, str: &str) {
        self.content.push_str(str);
        self.overflow_refresh();
    }

    #[inline]
    pub fn set_content(&mut self, str: &str) {
        self.content.set(str);
        self.overflow_refresh();
    }

    #[inline]
    pub fn set_placeholder(&mut self, str: &str) {
        self.placeholder.set(str);
    }

    pub fn truncate(&mut self) -> io::Result<String> {
        let truncate_pos = self.cursor_pos()?;
        let res_str = self.content.truncate(truncate_pos);
        self.overflow_refresh();
        return Ok(res_str);
    }

    #[inline]
    pub fn cursor_pos(&self) -> io::Result<usize> {
        let value = Cursor::pos_col()? + self.overflow_left - self.margin_left;
        return Ok(value);
    }

    #[inline]
    pub fn content(&self) -> &str {
        self.content.get()
    }

    pub fn clear(&mut self) {
        self.overflow_left = 0;
        self.overflow_right = 0;
        self.content.clear();
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.content.len()
    }
}
