#![feature(question_mark)]
extern crate space_toml;
extern crate argonaut;
#[macro_use]
extern crate glorious;
extern crate sdl2;
extern crate sdl2_image;

mod common;
mod editor;
mod start;

use std::env;
use argonaut::{ArgDef, Parse};
use start::start_editor;

const USAGE: &'static str = "Usage: cargo run \
    [--help | OPTIONS ] schema";

const HELP: &'static str = "\
Required arguments:
    schema                  A TOML file describing tiles and layers.

Optional arguments:
    --help | -h         Show this message.
    --version           Show the version of this library.
    --load PATH         Load this TOML file as a level.
    --save PATH         Save the level to this path.
    --edit PATH         The same as --load PATH --save PATH.
";

fn main() {
    use argonaut::Arg::*;

    // Create the arguments
    let a_schema = ArgDef::positional("schema");
    let a_version = ArgDef::named("version").switch();
    let a_help = ArgDef::named_and_short("help", 'h').switch();
    let a_load = ArgDef::named("load").option();
    let a_save = ArgDef::named("save").option();
    let a_edit = ArgDef::named("edit").option();

    let args: Vec<_> = env::args().skip(1).collect();

    // Prepare the options
    let mut schema = "";
    let mut load = None;
    let mut save = None;
    let mut edit = None;

    let expected = &[a_schema, a_version, a_help, a_load, a_save, a_edit];

    let mut parse = Parse::new(expected, &args).expect("Invalid definitions");
    while let Some(item) = parse.next() {
        match item {
            Err(_) => {
                // println!("Parse error: {:?}", err);
                println!("{}", USAGE);
                return;
            }
            Ok(Positional("schema", value)) => {
                schema = value;
            }
            Ok(Option("load", value)) => {
                load = Some(value);
            }
            Ok(Option("save", value)) => {
                save = Some(value);
            }
            Ok(Option("edit", value)) => {
                edit = Some(value);
            }
            Ok(Switch("help")) => {
                return println!("{}\n\n{}", USAGE, HELP);
            }
            Ok(Switch("version")) => {
                return println!("{}", env!("CARGO_PKG_VERSION"));
            }
            _ => unreachable!(),
        }
    }

    // Use the variables holding the parsed values for something
    println!("Parsed succesfully!");
    println!("Schema: {}", schema);
    println!("Load: {:?}", load);
    println!("Save: {:?}", save);
    println!("Edit: {:?}", edit);

    if let Some(path) = edit {
        if load.is_some() {
            return println!("Error: --edit overwrites --load");
        }
        if save.is_some() {
            return println!("Error: --edit overwrites --save");
        }
        load = Some(path);
        save = Some(path);
    }

    start_editor(schema, load, save);
}
