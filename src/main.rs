#![feature(question_mark)]
extern crate argonaut;
#[macro_use]
extern crate glorious;
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;
extern crate toml;

use std::path::Path;

use launch::start_editor;
use parse::{parse, read_options};
use schema::Schema;

mod common;
mod editor;
mod grid;
mod launch;
mod level;
mod parse;
mod schema;
mod toolbox;

fn main() {
    parse(|schema, load_from, save_to| {
        let schema = Schema::load(&Path::new(schema)).expect("Could not load schema");
        // read_options(schema, load_from, save_to);
        start_editor();
    });
}
