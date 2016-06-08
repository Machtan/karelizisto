use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::path::Path;

use json;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Point(pub i32, pub i32, pub u32);

impl PartialEq for Point {
    #[inline]
    fn eq(&self, other: &Point) -> bool {
        (self.0, self.1) == (other.0, other.1)
    }
}

impl Eq for Point {}

impl PartialOrd for Point {
    #[inline]
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    #[inline]
    fn cmp(&self, other: &Point) -> Ordering {
        (self.0, self.1).cmp(&(other.0, other.1))
    }
}

pub type Layer = BTreeMap<String, BTreeSet<Point>>;

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
