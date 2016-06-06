#![feature(custom_derive)]
#![feature(plugin)]
#![feature(question_mark)]

#![plugin(serde_macros)]

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate serde;
extern crate toml;

extern crate argonaut;
#[macro_use]
extern crate glorious;
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

use std::env;

use launch::start_editor;
use level::Level;
use schema::Schema;

mod common;
mod editor;
mod grid;
mod launch;
mod level;
mod schema;
mod toolbox;

fn main() {
    // Set up logging.

    let mut builder = env_logger::LogBuilder::new();
    builder.format(|record| {
        format!("[{}][{}] {}",
                record.level(),
                record.location().module_path(),
                record.args())
    });

    // Set default level to debug.
    // (Setting this before `parse`, makes it be considered *after* env vars (for now).)
    builder.filter(Some("karelizisto"), log::LogLevelFilter::Debug);
    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse(&var);
    }
    builder.init().unwrap();

    // Main

    let schema = Schema::load("schema.toml").unwrap();
    let level = Level::load("level.toml").unwrap();
    println!("{:#?}", level);
    start_editor(schema);
}
