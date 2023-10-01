use std::collections::VecDeque;

use super::event::EditorEvent;

#[derive(Debug)]
pub struct EditorHistory {
    events: VecDeque<EditorEvent>,

    // events that is undone
    undo_events: Vec<EditorEvent>,
    // events that is redone
    redo_events: Vec<EditorEvent>,
}

impl EditorHistory {
    const MAX_CACHED_EVENT: usize = 255;

    pub fn new() -> Self {
        Self {
            events: VecDeque::<EditorEvent>::new(),
            undo_events: Vec::<EditorEvent>::new(),
            redo_events: Vec::<EditorEvent>::new(),
        }
    }

    #[inline]
    // returns the last appended event
    pub fn previous_event(&self) -> Option<&EditorEvent> {
        self.events.back()
    }

    pub fn undo(&mut self) -> Option<&EditorEvent> {
        let option_op = if !self.redo_events.is_empty() {
            self.redo_events.pop()
        } else {
            self.events.pop_back()
        };

        match option_op {
            Some(op) => {
                self.undo_events.push(op);
                self.undo_events.last()
            }
            None => None,
        }
    }

    pub fn redo(&mut self) -> Option<&EditorEvent> {
        match self.undo_events.pop() {
            Some(op) => {
                self.redo_events.push(op);
                self.redo_events.last()
            }
            None => None,
        }
    }

    pub fn append(&mut self, ev: EditorEvent) {
        self.undo_events.clear();
        self.redo_events.clear();
        self.events.push_back(ev);

        if self.events.len() > Self::MAX_CACHED_EVENT {
            self.events.pop_front();
        }
    }
}
