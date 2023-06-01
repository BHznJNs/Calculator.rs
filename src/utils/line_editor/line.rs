use std::{rc::Rc, borrow::Borrow};

use crossterm::style::Stylize;
use super::tokenizer::{TokenVec, tokenize};

pub struct Line<'a> {
    content: &'a mut String,

    pub is_history: bool,
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

            is_history: false,
            label_width: label_str.len() + 1, // `3` is space width
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

    // borrow to use history content
    pub fn use_history(&mut self, content: Rc<String>) {
        self.is_history = true;
        self.tokens = tokenize(content.as_str());
    }
    // return to use self content
    pub fn reset(&mut self) {
        self.is_history = false;
        self.tokens = tokenize(self.content);
    }
    // use history content to replace self content
    pub fn reset_with(&mut self, mut new_content: String) {
        new_content.pop(); // pop the '\0' of the new_content

        *self.content = new_content;
        self.is_history = false;
        self.refresh();
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }
}