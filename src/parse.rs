// Disabled for now.

use std::env;
use std::fs::File;
use std::io::Read;

use argonaut::{ArgDef, Parse};
use toml;

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

pub fn parse<F>(continuation: F)
    where F: FnOnce(&str, Option<&str>, Option<&str>)
{
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
                // info!("Parse error: {:?}", err);
                info!("{}", USAGE);
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
                return info!("{}\n\n{}", USAGE, HELP);
            }
            Ok(Switch("version")) => {
                return info!("{}", env!("CARGO_PKG_VERSION"));
            }
            _ => unreachable!(),
        }
    }

    // Use the variables holding the parsed values for something
    info!("Parsed succesfully!");
    info!("Schema: {}", schema);
    info!("Load: {:?}", load);
    info!("Save: {:?}", save);
    info!("Edit: {:?}", edit);

    if let Some(path) = edit {
        if load.is_some() {
            return info!("Error: --edit overwrites --load");
        }
        if save.is_some() {
            return info!("Error: --edit overwrites --save");
        }
        load = Some(path);
        save = Some(path);
    }

    continuation(schema, load, save)
}
