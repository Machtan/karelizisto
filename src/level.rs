use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use serde::de::Deserialize;
use toml;

pub use schema::LoadError;

pub type Layer = BTreeMap<String, Vec<(u32, u32)>>;

#[derive(Clone, Debug)]
pub struct Level {
    pub name: String,
    pub schema: String,
    pub layers: BTreeMap<String, Layer>,
}

impl Level {
    pub fn load<P>(path: P) -> Result<Level, LoadError>
        where P: AsRef<Path>
    {
        let mut contents = String::new();

        File::open(&path)?.read_to_string(&mut contents)?;

        let mut parser = toml::Parser::new(&contents);

        let table = match parser.parse() {
            Some(table) => {
                for err in &parser.errors {
                    warn!("{}", err);
                }
                table
            }
            None => {
                let err = parser.errors.pop().unwrap();
                for err in &parser.errors {
                    warn!("{}", err);
                }
                return Err(LoadError::TomlParse(err));
            }
        };

        #[derive(Clone, Debug, Deserialize)]
        struct LevelHeader {
            name: String,
            schema: String,
        }

        let mut decoder = toml::Decoder::new(toml::Value::Table(table));
        let mut header = LevelHeader::deserialize(&mut decoder)?;

        let layers = if let Some(toml::Value::Table(table)) = decoder.toml {
            table.into_iter()
                .map(|(name, value)| {
                    let mut decoder = toml::Decoder::new(value);
                    let layer = Layer::deserialize(&mut decoder)?;
                    Ok((name, layer))
                })
                .collect::<Result<_, LoadError>>()?
        } else {
            BTreeMap::new()
        };

        Ok(Level {
            name: header.name,
            schema: header.schema,
            layers: layers,
        })
    }
}
