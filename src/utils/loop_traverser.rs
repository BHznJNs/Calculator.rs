use std::collections::VecDeque;

pub struct LoopTraverser<T> {
    vec: VecDeque<T>,
    pub index: isize,
    pub cycle: bool,
}

#[allow(dead_code)]
impl<T> LoopTraverser<T> {
    pub fn new(is_cycle: bool) -> Self {
        Self {
            vec: VecDeque::<T>::new(),
            index: -1,
            cycle: is_cycle,
        }
    }

    pub fn next(&mut self) -> Option<&T> {
        if self.vec.is_empty() || (!self.cycle && self.index == (self.vec.len() - 1) as isize) {
            return None;
        }

        self.index = (self.index + 1) % (self.vec.len() as isize);
        return self.current();
    }
    pub fn previous(&mut self) -> Option<&T> {
        if self.vec.is_empty() || (!self.cycle && (self.index == 0 || self.index == -1)) {
            if self.index == 0 {
                self.index = -1;
            }
            return None;
        }

        self.index = if self.index == 0 || self.index == -1 {
            (self.vec.len() - 1) as isize
        } else {
            (self.index - 1) % (self.vec.len() as isize)
        };
        return self.current();
    }

    #[inline]
    pub fn first(&self) -> Option<&T> {
        self.vec.front()
    }

    pub fn current(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        let result = if self.index == -1 {
            &self.vec[0]
        } else {
            &self.vec[self.index as usize]
        };
        return Some(result);
    }

    // --- --- --- --- --- ---

    #[inline]
    pub fn push_back(&mut self, element: T) {
        self.vec.push_back(element);
    }
    #[inline]
    pub fn push_front(&mut self, element: T) {
        self.vec.push_front(element);
    }

    // --- --- --- --- --- ---

    #[inline]
    pub fn reset_index(&mut self) {
        self.index = -1;
    }

    #[inline]
    pub fn set_content(&mut self, new_vec: Vec<T>) {
        self.vec = VecDeque::from(new_vec);
        self.reset_index();
    }

    #[inline]
    pub fn clear(&mut self) {
        self.vec.clear();
        self.reset_index();
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
}

#[test]
fn test() {
    let mut cycling_traverser = LoopTraverser::new(true);
    cycling_traverser.set_content(vec![1, 2, 3, 4]);

    // after `set_content` called, the `current` method must
    // returns the first value. 
    assert_eq!(cycling_traverser.current(), Some(&1));
    assert_eq!(cycling_traverser.previous(), Some(&4));
    cycling_traverser.reset_index();
    assert_eq!(cycling_traverser.next(), Some(&1));
    assert_eq!(cycling_traverser.next(), Some(&2));
    assert_eq!(cycling_traverser.next(), Some(&3));
    assert_eq!(cycling_traverser.next(), Some(&4));

    // --- --- --- --- --- ---

    let mut uncycling_traverser = LoopTraverser::new(false);
    uncycling_traverser.push_back(1);
    uncycling_traverser.push_back(2);
    uncycling_traverser.push_back(3);

    // after `push_back` called, the `current` method must
    // returns the first value. 
    assert_eq!(uncycling_traverser.current(), Some(&1));

    assert_eq!(uncycling_traverser.previous(), None);
    assert_eq!(uncycling_traverser.next(), Some(&1));
    assert_eq!(uncycling_traverser.next(), Some(&2));
    assert_eq!(uncycling_traverser.next(), Some(&3));
}
