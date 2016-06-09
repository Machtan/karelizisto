use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct SpriteSpec {
    pub texture: String,
    pub area: Option<(u32, u32, u32, u32)>,
}

#[derive(Deserialize)]
pub struct SchemaSpec {
    pub name: String,
    pub layers: Vec<String>,
    pub prefix: PathBuf,
    pub colors: Vec<String>,
    pub tiles: HashMap<String, SpriteSpec>,
}
