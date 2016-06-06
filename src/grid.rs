use std::collections::HashMap;

use sdl2::render::Texture;

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
