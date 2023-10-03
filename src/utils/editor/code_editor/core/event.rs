use super::super::cursor_pos::EditorCursorPos;

#[derive(Debug, Clone)]
pub enum EditorOperation {
    InsertChar(char),
    DeleteChar(char),
    InsertLine,
    DeleteLine,

    AppendIndent,
    RemoveIndent,

    //      from  , to
    Replace(String, String),
}

impl EditorOperation {
    pub fn rev(&self) -> Self {
        match self {
            Self::InsertChar(ch) => Self::DeleteChar(*ch),
            Self::DeleteChar(ch) => Self::InsertChar(*ch),
            Self::InsertLine => Self::DeleteLine,
            Self::DeleteLine => Self::InsertLine,

            Self::AppendIndent => Self::RemoveIndent,
            Self::RemoveIndent => Self::AppendIndent,

            Self::Replace(from, to) => Self::Replace(to.clone(), from.clone()),
        }
    }
}

// --- --- --- --- --- ---

#[derive(Debug, Clone)]
pub struct EditorEvent {
    pub op: EditorOperation,

    pub pos_before: EditorCursorPos,
    pub pos_after: EditorCursorPos,
}
