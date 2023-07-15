mod node;

use node::CompleterNode;

#[derive(Debug, PartialEq)]
pub struct Completer {
    root: CompleterNode,
}

impl Completer {
    pub fn new() -> Self {
        Completer {
            root: CompleterNode::new(),
        }
    }
    pub fn from(contents: Vec<String>) -> Self {
        let mut result = Self::new();
        for word in contents {
            result.insert(&word)
        }
        return result;
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
