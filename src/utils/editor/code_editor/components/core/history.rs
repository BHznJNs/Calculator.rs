use crate::utils::LoopTraverser;

pub struct ComponentHistory {
    list: LoopTraverser<String>,
    cached_content: String,

    // is using history content
    pub use_history: bool,
}

impl ComponentHistory {
    pub const HISTORY_PLACEHOLDER: &'static str = "Up & Down for history";

    pub fn new() -> Self {
        Self {
            list: LoopTraverser::new(false),
            cached_content: String::new(),
            use_history: false,
        }
    }

    pub fn next<'a>(&'a mut self) -> Option<&'a String> {
        let previous = self.list.previous();
        if previous.is_none() {
            self.use_history = false;
            return Some(&self.cached_content);
        } else {
            return previous;
        }
    }
    pub fn previous<'a>(&'a mut self) -> Option<&'a String> {
        self.use_history = true;
        self.list.next()
    }

    #[inline]
    pub fn set_cached(&mut self, content: String) {
        self.cached_content = content.to_owned();
    }

    #[inline]
    pub fn last<'a>(&'a self) -> Option<&'a String> {
        self.list.first()
    }

    #[inline]
    pub fn append(&mut self, element: String) {
        self.list.push_front(element);
    }
    #[inline]
    pub fn reset_index(&mut self) {
        self.cached_content.clear();
        self.list.reset_index();
    }
}
