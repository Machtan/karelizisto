extern crate glorious;

use super::common::{Message, State};
use glorious::Behavior;
use super::grid::GridManager;
use super::toolbox::Toolbox;

pub struct Editor {
    grid: GridManager,
    toolbox: Toolbox,
}

impl Editor {
    pub fn new(layers: Vec<String>, tiles: Vec<(String, String)>) -> Editor {
        let grid = GridManager::new(layers, tiles);
        let toolbox = Toolbox::new();
        Editor { 
            grid: grid,
            toolbox: toolbox,
        }
    }
}

impl Behavior<State> for Editor {
    type Message = Message;
}
