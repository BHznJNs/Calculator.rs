use crate::utils::editor::text_area::TextAreaContent;

use super::{token::{TokenVec, TokenSlicing}, tokenize};

pub struct TokenSequence {
    raw: String,
    tokens: TokenVec,
}

impl TextAreaContent for TokenSequence {
    fn new() -> Self {
        Self {
            raw: String::new(),
            tokens: TokenVec::new(),
        }
    }
    #[inline]
    fn get(&self) -> &String {
        &self.raw
    }
    #[inline]
    fn get_mut(&mut self) -> &mut String {
        &mut self.raw
    }
    #[inline]
    fn change_handler(&mut self) {
        self.tokens = tokenize(&self.raw);
    }

    fn rendered_content(&self, offset: usize, render_width: usize) -> String {
        let mut hidden_width = offset;
        let mut remain_width = render_width;
        let mut buffer = String::new();

        for token in &self.tokens {
            if remain_width == 0 {
                break;
            }

            if hidden_width > 0 {
                if hidden_width >= token.len() {
                    hidden_width -= token.len()
                } else {
                    let colored_token = token.get(..hidden_width);
                    let binding = colored_token.to_string();
                    let colored_chars = binding.chars();
                    buffer.extend(colored_chars);
                    remain_width -= token.len() - hidden_width;
                    hidden_width = 0;
                }
                continue;
            }

            if remain_width >= token.len() {
                let colored_token = token.get(..);
                let binding = colored_token.to_string();
                let colored_chars = binding.chars();
                buffer.extend(colored_chars);
                remain_width -= token.len();
            } else {
                let colored_token = token.get(.. (token.len() - remain_width));
                let binding = colored_token.to_string();
                let colored_chars = binding.chars();
                buffer.extend(colored_chars);
                remain_width = 0;
            }
        }

        if remain_width > 0 {
            buffer.extend(" ".repeat(remain_width).chars());
        }

        return buffer;
    }
}
