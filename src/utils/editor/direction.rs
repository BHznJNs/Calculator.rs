use crossterm::event::KeyCode;

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,

    Left,
    Right,
}

impl Direction {
    pub fn rev(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl From<KeyCode> for Direction {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Up => Self::Up,
            KeyCode::Down => Self::Down,
            KeyCode::Left => Self::Left,
            KeyCode::Right => Self::Right,
            _ => unreachable!(),
        }
    }
}
