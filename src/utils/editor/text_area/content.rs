use crate::utils::editor::tokenizer::TokenVec;

pub trait TextAreaContent {
    fn new() -> Self;
    fn get(&self) -> &String;
    fn get_mut(&mut self) -> &mut String;

    #[inline]
    fn tokens(&self) -> Option<&TokenVec> {
        None
    }
    #[inline]
    fn change_handler(&mut self) {}

    fn rendered_content(&self, offset: usize, render_width: usize) -> String {
        if self.len() > render_width {
            self.get()[offset..offset + render_width].to_owned()
        } else {
            let remain_width = render_width - self.len();
            let remain_space_str = " ".repeat(remain_width);
            self.get().to_owned() + &remain_space_str
        }
    }

    fn set(&mut self, str: &str) {
        *self.get_mut() = str.to_owned();
        self.change_handler();
    }

    fn insert(&mut self, pos: usize, ch: char) {
        self.get_mut().insert(pos, ch);
        self.change_handler();
    }
    fn remove(&mut self, pos: usize) -> char {
        let res_ch = self.get_mut().remove(pos);
        self.change_handler();
        return res_ch;
    }
    fn push_str(&mut self, str: &str) {
        self.get_mut().push_str(str);
        self.change_handler();
    }
    fn truncate(&mut self, new_len: usize) -> String {
        let mut res_str = String::new();
        self.get()[new_len..].clone_into(&mut res_str);
        self.get_mut().truncate(new_len);
        self.change_handler();
        return res_str;
    }
    fn clear(&mut self) {
        self.get_mut().clear();
        self.change_handler();
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.get().is_empty()
    }
    #[inline]
    fn len(&self) -> usize {
        self.get().len()
    }
}

impl TextAreaContent for String {
    #[inline]
    fn new() -> Self {
        String::new()
    }
    #[inline]
    fn get(&self) -> &String {
        self
    }
    #[inline]
    fn get_mut(&mut self) -> &mut String {
        self
    }
}
