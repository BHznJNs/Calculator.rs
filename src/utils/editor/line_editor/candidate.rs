pub struct Candidate {
    list: Vec<String>,
    index: isize,
}

impl Candidate {
    pub fn new() -> Self {
        Self {
            list: vec![],
            index: 0,
        }
    }

    pub fn set(&mut self, candidates: Vec<String>) {
        self.index = -1; // reset index
        self.list = candidates;
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn current_hint(&self) -> Option<&str> {
        if !self.list.is_empty() && self.index >= 0 {
            Some(&self.list[self.index as usize])
        } else {
            None
        }
    }
    pub fn next(&mut self) -> Option<&str> {
        let index = self.index;
        let len = self.list.len() as isize;

        if len > 0 {
            self.index = (index + 1) % len;
            Some(&self.list[self.index as usize])
        } else {
            None
        }
    }
}
