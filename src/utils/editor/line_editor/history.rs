use crate::utils::LoopTraverser;

pub struct EditorHistory {
    list: LoopTraverser<String>,
    cached_content: String,

    // is using history content
    pub use_history: bool,
}

impl EditorHistory {
    pub fn new() -> Self {
        Self {
            list: LoopTraverser::new(false),
            cached_content: String::new(),
            use_history: false,
        }
    }

    pub fn next(&mut self) -> Option<&String> {
        let previous = self.list.previous();
        if previous.is_none() {
            self.use_history = false;
            return Some(&self.cached_content);
        } else {
            return previous;
        }
    }
    pub fn previous(&mut self) -> Option<&String> {
        self.use_history = true;
        self.list.next()
    }

    #[inline]
    pub fn set_cached(&mut self, content: String) {
        self.cached_content = content.to_owned();
    }

    #[inline]
    pub fn append(&mut self, element: String) {
        self.reset_index();
        self.list.push_front(element);
    }

    #[inline]
    pub fn reset_index(&mut self) {
        self.cached_content.clear();
        self.list.reset_index();
    }
}
