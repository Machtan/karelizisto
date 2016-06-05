extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;
extern crate space_toml;

use super::common::{State, Message};
use glorious::{BoxedInputMapper, Game, Renderer, ResourceManager};
use sdl2::render::Texture;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::mouse::Mouse;
use sdl2_image::{INIT_JPG, INIT_PNG};
use std::rc::Rc;
use super::editor::Editor;

pub fn start_editor(schema_path: &str, load_from: Option<&str>, save_to: Option<&str>) {
    use sdl2::event::Event::*;
    use super::common::Message::*;
    
    println!("START!");
    
    
    
    // Load settings

    const WINDOW_TITLE: &'static str = "La bonega karelizisto";
    const WINDOW_SIZE: (u32, u32) = (800, 600);
    const MAX_FPS: u32 = 60;

    // Set up SDL2.

    let sdl_context = sdl2::init().expect("could not initialize SDL2");
    let video_subsystem = sdl_context.video().expect("could not initialize video subsystem");
    let _image_context = sdl2_image::init(INIT_PNG | INIT_JPG)
        .expect("could not initialize SDL2_image");
    let font_context = sdl2_ttf::init().expect("Font init");
    // let mut limiter = FrameLimiter::new(60);

    let window = video_subsystem.window(WINDOW_TITLE, WINDOW_SIZE.0, WINDOW_SIZE.1)
        .allow_highdpi()
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let (w, h) = window.size();
    let (pw, ph) = window.drawable_size();
    let mut renderer = window.renderer().build().unwrap();
    let _ = renderer.set_logical_size(w, h);

    let renderer = Renderer::new(renderer);
    let resources = ResourceManager::new(renderer.clone(), Rc::new(font_context));

    // Load units

    // Set up game state.

    let mut state = State::new();

    // Prepare the scene
    let layers = (&["Test 1", "Test 2"]).iter().map(|l| l.to_string()).collect();
    let units = (&[("raccoon", "raccoon.png")])
        .iter()
        .map(|&(name, tex)| (name.to_string(), tex.to_string()))
        .collect();
    let mut editor = Editor::new(layers, units);

    // Set up input handling.

    let mut mapper = BoxedInputMapper::new();

    mapper.add(map_event!(Quit { .. }, Exit));

    mapper.add(map_key_pressed!(Keycode::Up, Up));
    mapper.add(map_key_pressed!(Keycode::Down, Down));
    mapper.add(map_key_pressed!(Keycode::Left, Left));
    mapper.add(map_key_pressed!(Keycode::Right, Right));

    mapper.add(map_event!(
         MouseButtonDown { x, y, mouse_btn: Mouse::Left, .. },
         LeftClickAt((x * pw as i32) / w as i32, (y * ph as i32) / h as i32)
    ));
    mapper.add(map_event!(
         MouseButtonUp { x, y, mouse_btn: Mouse::Left, .. },
         LeftReleasedAt((x * pw as i32) / w as i32, (y * ph as i32) / h as i32)
    ));
    mapper.add(map_event!(
         MouseButtonDown { x, y, mouse_btn: Mouse::Right, .. },
         RightClickAt((x * pw as i32) / w as i32, (y * ph as i32) / h as i32)
    ));
    mapper.add(map_event!(
         MouseButtonUp { x, y, mouse_btn: Mouse::Right, .. },
         RightReleasedAt((x * pw as i32) / w as i32, (y * ph as i32) / h as i32)
    ));
    mapper.add(map_event!(
        MouseMotion { x, y, .. },
        MouseMovedTo((x * pw as i32) / w as i32, (y * ph as i32) / h as i32)
    ));

    // Run the main loop.

    let event_pump = sdl_context.event_pump().unwrap();
    let mut game = Game::new(MAX_FPS, renderer, event_pump);

    game.run(&mut state, &mapper, &mut editor, |m| *m == Exit);
}
