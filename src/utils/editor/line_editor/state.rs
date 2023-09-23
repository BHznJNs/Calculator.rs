pub struct LineState {
    pub left_end: bool,
    pub right_end: bool,
    pub line_start: bool,
    pub line_end: bool,
}

impl LineState {
    pub fn new() -> Self {
        Self {
            left_end: true,
            right_end: false,
            line_start: true,
            line_end: true,
        }
    }
}
