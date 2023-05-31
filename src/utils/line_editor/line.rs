use crossterm::style::Stylize;
use super::tokenizer::{TokenVec, tokenize};

pub struct Line<'a> {
    pub content: &'a mut String,
    pub label: String,
    pub label_width: usize,
    pub tokens: TokenVec,
}

impl<'a> Line<'a> {
    pub fn new(
        content: &'a mut String,
        line_count: usize,
        // term_width: usize,
        // prompt_width: usize,
    ) -> Self {
        let label_str = line_count.to_string();
        Line {
            content,
            label_width: label_str.len() + 1, // `1` is space width
            label: format!(" {}", label_str.black().on_white()),
            tokens: TokenVec::new(),
        }
    }

    fn refresh(&mut self) {
        // let total_width = self.prompt + self.content.len();

        // self.overflow_left = if total_width > term_width {
        //     total_width - term_width
        // } else { 0 };

        // token vector refresh
        self.content.push('\0');
        self.tokens = tokenize(self.content);
        self.content.pop();
    }

    // pub fn render(&self, terminal: &mut Terminal) -> io::Result<()> {
    //     terminal.cursor.hide()?;
    //     terminal.clear_line()?;

    //     let mut offset = self.overflow_left;
    //     let mut remain_space = self.term_width;
    //     for token in &self.tokens {
    //         if offset > 0 {
    //             if offset > token.len() {
    //                 offset -= token.len();
    //             } else {
    //                 remain_space -= token.len() - offset;
    //                 terminal.print::<&str>(&token.content[offset..], token.type__);
    //                 offset = 0;
    //             }
    //         } else {
    //             if token.len() <= remain_space {
    //                 remain_space -= token.len();
    //                 terminal.print::<&str>(&token.content, token.type__);
    //             } else {
    //                 terminal.print::<&str>(&token.content[..remain_space], token.type__);
    //             }
    //         }
    //     }

    //     terminal.flush()?;
    //     terminal.cursor.show()
    // }

    // push / pop
    pub fn push(&mut self, ch: char) {
        self.content.push(ch);
        self.refresh();
    }
    pub fn pop(&mut self) {
        self.content.pop();
        self.refresh();
    }

    // insert / remove
    pub fn insert(&mut self, index: usize, ch: char) {
        self.content.insert(index, ch);
        self.refresh();
        // self.content.insert(index + self.overflow_left, ch);
        // self.refresh();
    }
    pub fn remove(&mut self, index: usize) {
        self.content.remove(index);
        self.refresh();
        // self.content.remove(index + self.overflow_left);
        // self.refresh();
    }

    // --- --- --- --- --- ---

    // pub fn is_at_right_end(&self, terminal: &Terminal) -> io::Result<bool> {
    //     Ok((terminal.cursor_col()? + self.overflow_left + self.overflow_right) == self.len())
    // }

    pub fn len(&self) -> usize {
        self.content.len()
    }
}