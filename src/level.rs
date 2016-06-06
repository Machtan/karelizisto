use std::collections::BTreeMap;

use toml;

pub type Layer = BTreeMap<String, Vec<(u32, u32)>>;

#[derive(Clone, Debug)]
pub struct Level {
    pub name: String,
    pub schema: String,
    pub layers: BTreeMap<String, Layer>,
}


