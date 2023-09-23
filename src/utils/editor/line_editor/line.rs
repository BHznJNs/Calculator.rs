use std::rc::Rc;

use crate::public::env::ENV;
use crossterm::style::Stylize;

use super::tokenizer::{tokenize, TokenVec};

pub struct Line {
    pub content: String,

    pub is_history: bool,
    pub label: String,
    pub label_width: usize,
    pub tokens: TokenVec,
}

impl Line {
    pub fn new(line_count: usize) -> Self {
        let label_str = line_count.to_string();
        let label_fmted_width = label_str.len() + 1; // `1` is space width
        let label_fmted = if unsafe { ENV.options.support_ansi } {
            format!(" {}", label_str.black().on_white())
        } else {
            format!(" {}", label_str)
        };

        Self {
            content: String::new(),

            is_history: false,
            label_width: label_fmted_width,
            label: label_fmted,
            tokens: TokenVec::new(),
        }
    }

    fn refresh(&mut self) {
        // token vector refresh
        self.tokens = tokenize(&self.content);
    }

    // push / pop
    pub fn push(&mut self, ch: char) {
        // in: '('; pushed: "()"
        // in: '['; pushed: "[]"
        // in: '#'; pushed: '#'

        // if character is to paired
        let paired_ch = match ch {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '\'' | '\"' => ch,
            _ => '\0',
        };
        self.content.push(ch);

        // output this character with paired
        if paired_ch != '\0' {
            self.content.push(paired_ch);
        }
        self.refresh();
    }
    pub fn push_str(&mut self, str: &str) {
        self.content.push_str(str);
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
        // self.tokens = tokenize(self.content);
        self.refresh();
    }
    // use history content to replace self content
    pub fn reset_with(&mut self, new_content: String) {
        self.content = new_content;
        self.is_history = false;
        self.refresh();
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }
}
