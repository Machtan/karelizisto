use std::path::PathBuf;

use glorious::{BoxedInputMapper, Device, Game, ResourceManager};
use sdl2;
use sdl2::render::BlendMode;
use sdl2::keyboard::Scancode;
use sdl2::mouse::Mouse;
use sdl2_image::{self, INIT_JPG, INIT_PNG};
use sdl2_ttf;

use common::State;
use editor::Editor;
use level::Level;
use schema::{parse_color, Schema};

pub fn start_editor<P>(schema: Schema, level: Level, save_to: Option<P>)
    where P: Into<PathBuf>
{
    use sdl2::event::Event::*;
    use super::common::Message::*;

    info!("START!");

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
    let mut renderer = window.renderer().present_vsync().build().unwrap();
    renderer.set_blend_mode(BlendMode::Blend);
    let _ = renderer.set_logical_size(w, h);

    let device = Device::new(renderer);
    let renderer = device.create_renderer();
    let resources = ResourceManager::with_prefix(schema.prefix, &device, &font_context);

    // Load units

    // Set up game state.

    let mut state = State::new(resources);

    // Prepare the scene
    let colors = schema.colors.into_iter().map(|s| parse_color(&s)).collect::<Vec<_>>();
    let mut editor = Editor::new(schema.layers, schema.tiles, colors, level, save_to);

    // Set up input handling.

    let mut mapper = BoxedInputMapper::new();

    mapper.add(map_event!(Quit { .. }, PreExit));

    mapper.add(map_scan_pressed!(Scancode::Up, Up));
    mapper.add(map_scan_pressed!(Scancode::Left, Left));
    mapper.add(map_scan_pressed!(Scancode::Down, Down));
    mapper.add(map_scan_pressed!(Scancode::Right, Right));

    mapper.add(map_scan_pressed!(Scancode::W, Up));
    mapper.add(map_scan_pressed!(Scancode::A, Left));
    mapper.add(map_scan_pressed!(Scancode::S, Down));
    mapper.add(map_scan_pressed!(Scancode::D, Right));

    mapper.add(map_scan_pressed!(Scancode::Z, PrevLayer));
    mapper.add(map_scan_pressed!(Scancode::X, NextLayer));
    mapper.add(map_scan_pressed!(Scancode::Q, PrevTile));
    mapper.add(map_scan_pressed!(Scancode::E, NextTile));

    mapper.add(map_scan_pressed!(Scancode::Num1, PrevColor));
    mapper.add(map_scan_pressed!(Scancode::Num2, NextColor));

    mapper.add(map_scan_pressed!(Scancode::Return, Save));

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
