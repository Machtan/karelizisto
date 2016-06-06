#![feature(question_mark)]
extern crate space_toml;
extern crate argonaut;
#[macro_use]
extern crate glorious;
extern crate sdl2;
extern crate sdl2_image;

mod parse;
mod schema;
mod common;
mod level;
mod toolbox;
mod grid;
mod editor;
mod launch;

use std::path::Path;
use parse::{parse, read_options};
use launch::{start_editor};
use schema::Schema;

fn main() {
    parse(|schema, load_from, save_to| {
        let schema = Schema::load(&Path::new(schema)).expect("Could not load schema");
        //read_options(schema, load_from, save_to);
        start_editor();
    });
}
