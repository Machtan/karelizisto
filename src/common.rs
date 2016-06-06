use glorious::ResourceManager;

use toolbox::Tool;

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    Up,
    Down,
    Left,
    Right,
    PreExit,
    Exit,

    NextLayer,
    PrevLayer,
    NextTile,
    PrevTile,
    Save,

    LeftClickAt(i32, i32),
    LeftReleasedAt(i32, i32),
    RightClickAt(i32, i32),
    RightReleasedAt(i32, i32),
    MouseMovedTo(i32, i32),
}

pub struct State<'a> {
    pub resources: ResourceManager<'a, 'static>,
}

impl<'a> State<'a> {
    pub fn new(resources: ResourceManager<'a, 'static>) -> State<'a> {
        State { resources: resources }
    }
}
