
use std::path::{Path, PathBuf};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug, Clone)]
pub struct Schema {
    pub name: String,
    pub layers: Vec<String>,
    prefix: PathBuf,
    tiles: HashMap<String, String>,
}

#[derive(Debug)]
pub enum SchemaLoadError {
    MissingName,
    MissingTiles,
    NoTiles,
    MissingLayers,
    NoLayers,
    ReadError(io::Error),
}

impl From<io::Error> for SchemaLoadError {
    fn from(err: io::Error) -> SchemaLoadError {
        SchemaLoadError::ReadError(err)
    }
}

impl Schema {
    pub fn new(name: &str, layers: Vec<String>, prefix: &str, tiles: HashMap<String, String>) -> Schema {
        Schema {
            name: name.to_string(),
            prefix: PathBuf::from(prefix),
            layers: layers,
            tiles: tiles,
        }
    }
    
    pub fn load(path: &Path) -> Result<Schema, SchemaLoadError> {
        unimplemented!();
    }
}
/*
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
*/