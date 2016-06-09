use std::cmp;
use std::fmt::{self, Display, Write as FmtWrite};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use serde::Deserialize;
use toml;

#[derive(Debug)]
pub enum Error {
    Read(io::Error),
    Parse,
    Decode(toml::DecodeError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Read(ref err) => write!(f, "error reading file: {}", err),
            Error::Parse => write!(f, "error parsing file"),
            Error::Decode(ref err) => write!(f, "error decoding file: {}", err),
        }
    }
}

fn warn_about_unused_keys<F>(value: toml::Value, path: &mut String, warn: &mut F) -> bool
    where F: FnMut(&str)
{
    let mut result = false;
    if let toml::Value::Table(table) = value {
        for (key, child) in table {
            result = true;
            let i = path.len();
            if !path.is_empty() {
                path.push('.');
            }
            path.push_str(&key);
            if !warn_about_unused_keys(child, path, warn) {
                warn(&format!("unused key in info file: {:?}", path));
            }
            path.truncate(i);
        }
    }
    result
}

fn make_context(err: &toml::ParserError, contents: &str, ctx: &mut String) {
    let i = contents[..err.lo + 1].rfind('\n').unwrap_or(!0).wrapping_add(1);
    let j = err.lo + contents[err.lo..].find('\n').unwrap_or(contents.len() - err.lo);
    let line_num = contents[..i].split('\n').count() + 1;
    let line = &contents[i..j];

    let trailing = if j < err.hi {
        "..."
    } else {
        ""
    };
    let prev_len = ctx.len();
    write!(ctx, "{} > ", line_num).unwrap();
    let prefix_len = ctx.len() - prev_len;
    write!(ctx, "{}{}\n", line, trailing).unwrap();
    for _ in 0..(prefix_len + err.lo - i) {
        ctx.push(' ');
    }
    for _ in 0..cmp::max(cmp::min(err.hi - err.lo, j - err.lo), 1) {
        ctx.push('~');
    }
}

pub fn load_toml<T, P, F>(path: P, mut warn: F) -> Result<T, Error>
    where T: Deserialize,
          P: AsRef<Path>,
          F: FnMut(&str)
{
    let mut contents = String::new();
    File::open(path)
        .and_then(|mut file| file.read_to_string(&mut contents))
        .map_err(Error::Read)?;
    let mut parser = toml::Parser::new(&contents);

    let table = parser.parse();
    for warning in &parser.errors {
        let mut msg = format!("parsing file: {}\n", warning);
        make_context(warning, &contents, &mut msg);
        warn(&msg);
    }
    let table = table.ok_or(Error::Parse)?;

    let mut decoder = toml::Decoder::new(toml::Value::Table(table));
    let spec = T::deserialize(&mut decoder).map_err(Error::Decode)?;

    if let Some(value) = decoder.toml {
        let mut path = String::new();
        warn_about_unused_keys(value, &mut path, &mut warn);
    }
    Ok(spec)
}
