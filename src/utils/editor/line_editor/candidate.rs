use crate::utils::loop_traverser::LoopTraverser;

pub struct Candidate {
    list: LoopTraverser<String>,
}

impl Candidate {
    pub fn new() -> Self {
        Self {
            list: LoopTraverser::new(true),
        }
    }

    pub fn set(&mut self, candidates: Vec<String>) {
        self.list.set_content(candidates);
        self.next();
    }

    #[inline]
    pub fn current(&self) -> Option<&String> {
        self.list.current()
    }
    #[inline]
    pub fn next(&mut self) -> Option<&String> {
        self.list.next()
    }
    #[inline]
    pub fn previous(&mut self) -> Option<&String> {
        self.list.previous()
    }

    pub fn clear(&mut self) {
        self.list.clear()
    }
}
