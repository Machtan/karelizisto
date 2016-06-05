
use self::glorious::
use std::path::Path;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Schema {
    pub name: String,
    pub layers: Vec<String>,
    prefix: Path,
    tiles: HashMap<String, String>,
}

impl Schema {
    pub fn new(name: &str, layers: Vec<String>, prefix: &str, tiles: HashMap<(String, String)>) -> Schema {
        Schema {
            name: name.to_string(),
            prefix: Path::new(prefix),
            layers: layers,
            tiles: tiles,
        }
    }
}

name = "protoboard"
layers = ["terrain", "units"]
prefix = "../protoboard/assets"

[tiles]
protector = "shield.png"
archer = "bow.png"
warrior = "sword.png"
raccoon = "raccoon.png"

mountains = "mountains.png"
woods = "woods.png"