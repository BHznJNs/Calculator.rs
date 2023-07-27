mod node;

use node::CompleterNode;

#[derive(Debug, PartialEq)]
pub struct Completer {
    root: CompleterNode,
}

impl Completer {
    pub fn new() -> Self {
        Self {
            root: CompleterNode::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        self.root.insert(word.chars());
    }

    pub fn complete(&self, word: &str) -> Vec<String> {
        let mut result = Vec::<String>::new();
        self.root.complete(word.chars(), &mut result);
        return result;
    }
}

impl<'a> Extend<&'a str> for Completer {
    fn extend<T: IntoIterator<Item = &'a str>>(&mut self, iter: T) {
        for item in iter {
            self.insert(&item);
        }
    }
}
