use std::{collections::BTreeMap, str::Chars};

#[derive(Debug)]
pub struct CompleterNode {
    subnodes: BTreeMap<char, CompleterNode>,
}

impl CompleterNode {
    pub fn new() -> Self {
        Self {
            subnodes: BTreeMap::<char, CompleterNode>::new(),
        }
    }

    pub fn insert(&mut self, mut word: Chars) {
        match word.next() {
            Some(ch) => {
                let subnode = self.subnodes.entry(ch).or_insert(Self::new());
                subnode.insert(word);
            }
            None => {
                self.subnodes.insert(0 as char, Self::new());
            }
        }
    }

    pub fn complete(&self, mut word: Chars, result: &mut Vec<String>) {
        match word.next() {
            Some(ch) => {
                if let Some(node) = self.subnodes.get(&ch) {
                    node.complete(word, result)
                }
            }
            None => {
                for (ch, subnode) in &self.subnodes {
                    if *ch == '\0' { continue };
                    subnode.collect(String::from(*ch), result);
                }
            }
        }
    }

    fn collect(&self, partial: String, result: &mut Vec<String>) {
        if self.subnodes.is_empty() {
            // no subnode
            result.push(partial)
        } else {
            for (ch, subnode) in &self.subnodes {
                if *ch == '\0' {
                    result.push(partial.clone())
                } else {
                    let mut cloned = partial.clone();
                    cloned.push(*ch);
                    subnode.collect(cloned, result);
                }
            }
        }
    }
}
