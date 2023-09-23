use std::collections::VecDeque;

pub struct LoopTraverser<T> {
    vec: VecDeque<T>,
    pub index: isize,
    pub cycle: bool,
}

impl<T> LoopTraverser<T> {
    pub fn new(is_cycle: bool) -> Self {
        Self {
            vec: VecDeque::<T>::new(),
            index: -1,
            cycle: is_cycle,
        }
    }

    pub fn next<'a>(&'a mut self) -> Option<&'a T> {
        if self.vec.is_empty() || (!self.cycle && self.index == (self.vec.len() - 1) as isize) {
            return None;
        }

        self.index = (self.index + 1) % (self.vec.len() as isize);
        return Some(self.current());
    }
    pub fn previous<'a>(&'a mut self) -> Option<&'a T> {
        if self.vec.is_empty() || (!self.cycle && (self.index == 0 || self.index == -1)) {
            return None;
        }

        self.index = if self.index == 0 || self.index == -1 {
            (self.vec.len() - 1) as isize
        } else {
            (self.index - 1) % (self.vec.len() as isize)
        };
        return Some(self.current());
    }

    #[inline]
    pub fn first<'a>(&'a self) -> Option<&'a T> {
        self.vec.front()
    }

    #[inline]
    pub fn current<'a>(&'a self) -> &'a T {
        if self.index == -1 {
            &self.vec[0]
        } else {
            &self.vec[self.index as usize]
        }
    }

    // --- --- --- --- --- ---

    #[inline]
    pub fn push_back(&mut self, element: T) {
        self.vec.push_back(element);
    }
    // #[inline]
    // pub fn push_front(&mut self, element: T) {
    //     self.vec.push_front(element);
    // }
    // #[inline]
    // pub fn pop_back(&mut self) -> Option<T> {
    //     self.vec.pop_back()
    // }
    // #[inline]
    // pub fn pop_front(&mut self) -> Option<T> {
    //     self.vec.pop_front()
    // }

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

    // #[inline]
    // pub fn len(&self) -> usize {
    //     self.vec.len()
    // }
}

#[test]
fn test() {
    let mut cycling_traverser = LoopTraverser::new(true);
    cycling_traverser.set_content(vec![1, 2, 3, 4]);

    assert_eq!(cycling_traverser.previous(), Some(&4));
    cycling_traverser.reset_index();
    assert_eq!(cycling_traverser.next(), Some(&1));
    assert_eq!(cycling_traverser.next(), Some(&2));
    assert_eq!(cycling_traverser.next(), Some(&3));
    assert_eq!(cycling_traverser.next(), Some(&4));

    // --- --- --- --- --- ---

    let mut uncycling_traverser = LoopTraverser::new(false);
    uncycling_traverser.set_content(vec![1, 2, 3, 4]);

    assert_eq!(uncycling_traverser.previous(), None);
    assert_eq!(uncycling_traverser.next(), Some(&1));
    assert_eq!(uncycling_traverser.next(), Some(&2));
    assert_eq!(uncycling_traverser.next(), Some(&3));
    assert_eq!(uncycling_traverser.next(), Some(&4));
    assert_eq!(uncycling_traverser.next(), None);
    assert_eq!(uncycling_traverser.previous(), Some(&3));
}
