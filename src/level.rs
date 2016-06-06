use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;

use json;
use serde::{Deserialize, Serialize};

pub use schema::LoadError;

pub type Layer = BTreeMap<String, Vec<(u32, u32)>>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Level {
    pub name: String,
    pub schema: String,
    pub layers: BTreeMap<String, Layer>,
}

impl Level {
    #[inline]
    pub fn load<P>(path: P) -> Result<Level, LoadError>
        where P: AsRef<Path>
    {
        Ok(json::from_reader(File::open(path)?)?)
    }
}
