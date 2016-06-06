use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::path::Path;

use json;

pub type Layer = BTreeMap<String, BTreeSet<(i32, i32)>>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Level {
    pub name: String,
    pub schema: String,
    pub layers: BTreeMap<String, Layer>,
}

impl Level {
    #[inline]
    pub fn load<P>(path: P) -> Result<Level, json::Error>
        where P: AsRef<Path>
    {
        json::from_reader(File::open(path)?)
    }

    #[inline]
    pub fn save<P>(&self, path: P) -> Result<(), json::Error>
        where P: AsRef<Path>
    {
        json::to_writer(&mut File::create(path)?, self)
    }
}
