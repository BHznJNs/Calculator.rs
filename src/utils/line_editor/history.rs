use std::rc::Rc;

pub struct History {
    index: usize,
    content_list: Vec<Rc<String>>,
}

impl History {
    pub fn new() -> Self {
        History {
            index: 0,
            content_list: vec![],
        }
    }

    pub fn previous(&mut self) -> Option<Rc<String>> {
        let index = self.index;
        let content_len = self.content_list.len();

        if index < content_len {
            self.index += 1;
            Some(self.content_list[content_len - index - 1].clone())
        } else {
            None
        }
    }
    pub fn next(&mut self) -> Option<Rc<String>> {
        if self.index > 0 {
            self.index -= 1;
        }

        if self.index > 0 {
            Some(self.content_list[self.content_list.len() - self.index].clone())
        } else {
            None
        }
    }

    // --- --- --- --- --- ---

    pub fn reset_index(&mut self) {
        self.index = 0;
    }

    pub fn get_current(&self) -> Option<String> {
        let (index, content_list) = (self.index, &self.content_list);

        if index != 0 {
            let current_candidate_ref = content_list[content_list.len() - index].as_ref();
            Some(current_candidate_ref.clone())
        } else {
            None
        }
    }

    pub fn append(&mut self, mut line_content: String) {
        self.index = 0;
        // string end with '\0' is for tokenizer
        line_content.push('\0');

        self.content_list.push(Rc::new(line_content));
    }
}
