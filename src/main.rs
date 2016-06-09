#![feature(custom_derive)]
#![feature(plugin)]
#![feature(question_mark)]

#![plugin(serde_macros)]

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json as json;
extern crate toml;

extern crate argonaut;
#[macro_use]
extern crate glorious;
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

use std::env;

use info::Schema;
use launch::start_editor;
use level::Level;
use load::load_toml;

mod common;
mod editor;
mod info;
mod launch;
mod level;
mod load;
mod spec;
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

    let schema = match load_toml("schema.toml", |m| warn!("{}", m)) {
        Ok(spec) => Schema::from_spec(spec).expect("could not validate schema"),
        Err(err) => panic!("could not load schema: {}", err),
    };
    let level = Level::load("level.json").unwrap();
    assert!(level.schema == schema.name);

    start_editor(schema, level, Some("level-output.json"));
}
