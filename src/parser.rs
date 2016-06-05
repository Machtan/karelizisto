extern crate argonaut;

use std::env;
use argonaut::{ArgDef, Parse};
use space_toml::TomlValue;
use std::fs::File;
use std::io::Read;

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

fn parse() {
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
    
    start(schema, load, save);
}

fn read_options(schema_path: &str, load_from: Option<&str>, save_to: Option<&str>) {
    // Load the schema file
    let mut file = match File::open(schema_path) {
        Ok(file) => file,
        Err(e) => {
            return println!("Could not read schema file at {:?}: {:?}", schema_path, e);
        }
    };
    let mut schema_toml = String::new();
    file.read_to_string(&mut schema_toml).expect("Could not read schema");
    let schema = space_toml::parse(&schema_toml).expect("Could not parse TOML");
    let schema_name = schema.get("name").and_then(|t| t.string()).expect("Invalid name in schema");
    
    // Load the level
    let mut level_toml = String::new();
    let level_table = if let Some(path) = load_from {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                return println!("Could not read level file at {:?}: {:?}", path, e);
            }
        };
        file.read_to_string(&mut level_toml).expect("Could not read level file");
        Some(space_toml::parse(&level_toml).expect("Could not parse TOML"))
    } else {
        None
    };
    if let Some(table) = level_table {
        if let Some(level_schema) = table.get("schema").and_then(|t| t.string()) {
            if level_schema != schema_name {
                return println!("The level isn't valid for this schema");
            }
        } else {
            return println!("No schema specified in the level file");
        }
    }
}