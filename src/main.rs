#![feature(question_mark)]
extern crate space_toml;
extern crate argonaut;
#[macro_use]
extern crate glorious;
extern crate sdl2;
extern crate sdl2_image;

mod libui;

fn main() {
    libui::start_editor();
}
