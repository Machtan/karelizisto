
#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    Up,
    Down,
    Left,
    Right,
    Exit,

    LeftClickAt(i32, i32),
    LeftReleasedAt(i32, i32),
    RightClickAt(i32, i32),
    RightReleasedAt(i32, i32),
    MouseMovedTo(i32, i32),
}

pub struct State {
    test: bool,
}

impl State {
    pub fn new() -> State {
        State { test: false }
    }
}
