
use common::{Message, State};
use glorious::Behavior;
use sdl2::render::Texture;
use std::collections::HashMap;

pub struct GridManager {
    layers: Vec<String>,
    tiles: HashMap<String, Texture>,
    contents: HashMap<String, HashMap<(u32, u32), String>>,
}

impl GridManager {
    pub fn new(layers: Vec<String>, tiles: Vec<(String, String)>) -> GridManager {
        let mut tiles = HashMap::new();
        let mut contents = HashMap::new();
        GridManager {
            layers: layers,
            tiles: tiles,
            contents: contents,
        }
    }
}

pub struct Editor {
    grid: GridManager,
}

impl Editor {
    pub fn new(layers: Vec<String>, tiles: Vec<(String, String)>) -> Editor {
        let grid = GridManager::new(layers, tiles);
        Editor { grid: grid }
    }
}

impl Behavior<State> for Editor {
    type Message = Message;
}
