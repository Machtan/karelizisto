use common::{Message, State};
use glorious::Behavior;
use grid::GridManager;

pub struct Editor {
    grid: GridManager,
}

impl Editor {
    pub fn new(layers: Vec<String>, tiles: Vec<(String, String)>) -> Editor {
        let grid = GridManager::new(layers, tiles);
        Editor { grid: grid }
    }
}

impl<'a> Behavior<State<'a>> for Editor {
    type Message = Message;
}
