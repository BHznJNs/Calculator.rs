use crossterm::style::Stylize;
use super::tokenizer::{TokenVec, tokenize};

pub struct Line<'a> {
    content: &'a mut String,
    history_content: Option<&'a str>,

    pub label: String,
    pub label_width: usize,
    pub tokens: TokenVec,
}

impl<'a> Line<'a> {
    pub fn new(
        content: &'a mut String,
        line_count: usize,
    ) -> Self {
        let label_str = line_count.to_string();
        Line {
            content,
            history_content: None,
            label_width: label_str.len() + 1, // `1` is space width
            label: format!(" {}", label_str.black().on_white()),
            tokens: TokenVec::new(),
        }
    }

    fn refresh(&mut self) {
        // token vector refresh
        self.content.push('\0');
        self.tokens = tokenize(self.content);
        self.content.pop();
    }

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
    }
    pub fn remove(&mut self, index: usize) {
        self.content.remove(index);
        self.refresh();
    }

    // --- --- --- --- --- ---

    pub fn reset_with(&mut self, content: &'a str) {
        self.history_content = Some(content);
        self.refresh();
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }
}