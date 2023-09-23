use std::io;

use crossterm::style::Stylize;

use crate::utils::{Cursor, Terminal};

use super::direction::Direction;
#[derive(Clone)]

pub struct TextArea {
    content: String,
    placeholder: String,

    pub margin_left: usize,
    pub margin_right: usize,

    overflow_left: usize,
    overflow_right: usize,
}

pub struct TextAreaStateLeft {
    pub is_at_left_side: bool,
    pub is_at_area_start: bool,
}
pub struct TextAreaStateRight {
    pub is_at_right_side: bool,
    pub is_at_area_end: bool,
}

// state calculating methods
impl TextArea {
    #[inline]
    pub fn is_at_left_side(&self) -> io::Result<bool> {
        return Ok(Cursor::pos_col()? == self.margin_left);
    }
    #[inline]
    pub fn is_at_right_side(&self) -> io::Result<bool> {
        return Ok(Cursor::pos_col()? == Terminal::width() - 1);
    }

    pub fn state_left(&self) -> io::Result<TextAreaStateLeft> {
        let is_at_left_side = self.is_at_left_side()?;
        let is_at_area_start = is_at_left_side && self.overflow_left == 0;
        return Ok(TextAreaStateLeft {
            is_at_left_side,
            is_at_area_start,
        });
    }
    pub fn state_right(&self) -> io::Result<TextAreaStateRight> {
        let cursor_pos_col = Cursor::pos_col()?;
        let is_at_right_side = self.is_at_right_side()?;
        let is_at_area_end = cursor_pos_col == (self.len() + self.margin_left)
            || cursor_pos_col == (self.len() - self.overflow_left + self.margin_left);
        return Ok(TextAreaStateRight {
            is_at_right_side,
            is_at_area_end,
        });
    }
}

// editing methods
impl TextArea {
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
        match dir {
            Direction::Left => {
                let state = self.state_left()?;
                if state.is_at_area_start {
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
                if state.is_at_area_end {
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
}

impl TextArea {
    pub fn new(margin_left: usize, margin_right: usize) -> Self {
        Self {
            content: String::new(),
            placeholder: String::new(),

            overflow_left: 0,
            overflow_right: 0,

            margin_left,
            margin_right,
        }
    }

    pub fn render(&self) -> io::Result<()> {
        let visible_area_width = self.visible_area_width();

        let rendered_content = if self.len() == 0 && !self.placeholder.is_empty() {
            if self.placeholder.len() > visible_area_width {
                let rendered_range = 0..visible_area_width;
                self.placeholder[rendered_range].dim()
            } else {
                self.placeholder.as_str().dim()
            }
        } else {
            if self.len() > visible_area_width {
                let rendered_range = self.overflow_left..(self.len() - self.overflow_right);
                self.content[rendered_range].stylize()
            } else {
                self.content.as_str().stylize()
            }
        };
        let remain_area_width = visible_area_width - rendered_content.content().len();
        let remain_space_str = " ".repeat(remain_area_width);

        let saved_cursor_pos = Cursor::pos_col()?;
        Cursor::move_to_col(self.margin_left)?;
        print!("{}{}", rendered_content, remain_space_str);
        Terminal::flush()?;
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
        if self.state_left()?.is_at_area_start {
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

    #[inline]
    pub fn push_str(&mut self, str: &str) {
        self.content.push_str(str);
        self.overflow_refresh();
    }

    #[inline]
    pub fn set_content(&mut self, str: &str) {
        self.content = str.to_owned();
        self.overflow_refresh();
    }

    #[inline]
    pub fn set_placeholder(&mut self, str: &str) {
        self.placeholder = str.to_owned();
    }

    pub fn truncate(&mut self) -> io::Result<String> {
        let truncate_pos = self.cursor_pos()?;
        let mut res_str = String::new();

        self.content[truncate_pos..].clone_into(&mut res_str);
        self.content.truncate(truncate_pos);
        self.overflow_refresh();
        return Ok(res_str);
    }

    #[inline]
    pub fn cursor_pos(&self) -> io::Result<usize> {
        let value = Cursor::pos_col()? + self.overflow_left - self.margin_left;
        return Ok(value);
    }

    #[inline]
    pub fn content<'a>(&'a self) -> &'a str {
        &self.content
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
