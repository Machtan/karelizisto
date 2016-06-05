#![feature(question_mark)]
extern crate space_toml;
extern crate argonaut;
#[macro_use]
extern crate glorious;
extern crate sdl2;
extern crate sdl2_image;

mod parse;
mod common;
mod level;
mod toolbox;
mod grid;
mod editor;
mod launch;

use parse::{parse, read_options};
use launch::{start_editor};

fn main() {
    parse(|schema, load_from, save_to| {
        read_options(schema, load_from, save_to);
        start_editor();
    });
}
