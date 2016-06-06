use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use serde::de::Deserialize;
use toml;

#[derive(Clone, Debug, Deserialize)]
pub struct Schema {
    pub name: String,
    pub layers: Vec<String>,
    pub prefix: PathBuf,
    pub tiles: BTreeMap<String, String>,
}

#[derive(Debug)]
pub enum LoadError {
    Read(io::Error),
    TomlParse(toml::ParserError),
    TomlDecode(toml::DecodeError),
}

impl From<io::Error> for LoadError {
    #[inline]
    fn from(err: io::Error) -> LoadError {
        LoadError::Read(err)
    }
}

impl From<toml::DecodeError> for LoadError {
    #[inline]
    fn from(err: toml::DecodeError) -> LoadError {
        LoadError::TomlDecode(err)
    }
}

impl Schema {
    pub fn load<P>(path: P) -> Result<Schema, LoadError>
        where P: Into<PathBuf>
    {
        let path = path.into();
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

        let mut decoder = toml::Decoder::new(toml::Value::Table(table));

        let mut schema = Schema::deserialize(&mut decoder)?;
        if let Some(rest) = decoder.toml {
            for (key, _) in rest.as_table().unwrap() {
                warn!("ignoring key {:?} in schema", key);
            }
        }

        // Make prefix relative to schema.
        let mut prefix = path;
        prefix.pop();
        prefix.push(&schema.prefix);
        schema.prefix = prefix;

        Ok(schema)
    }
}
