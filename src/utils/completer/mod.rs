mod node;

use node::CompleterNode;

#[derive(Debug)]
pub struct Completer {
    root: CompleterNode,
}

impl Completer {
    pub fn new() -> Self {
        Completer { root: CompleterNode::new() }
    }
    pub fn insert(&mut self, word: &str) {
        self.root.insert(word.chars());
    }

    pub fn complete(&mut self, word: &str) -> Vec<String> {
        let mut result = Vec::<String>::new();
        self.root.complete(word.chars(), &mut result);
        return result;
    }
}