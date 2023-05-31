pub struct History {
    index: usize,
    content_list: Vec<String>
}

impl History {
    pub fn new() -> Self {
        History {
            index: 0,
            content_list: vec![]
        }
    }

    pub fn previous(&mut self) -> Option<&String> {
        if self.index > 0 {
            self.index -= 1;
            Some(&self.content_list[self.index - 1])
        } else {
            None
        }
    }
    pub fn next(&mut self) -> Option<&String> {
        if self.index < self.content_list.len() - 1 {
            self.index -= 1;
            Some(&self.content_list[self.index + 1])
        } else {
            None
        }
    }

    pub fn append(&mut self, line_content: String) {
        self.index += 1;
        self.content_list.push(line_content);
    }
}