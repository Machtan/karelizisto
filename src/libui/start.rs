extern crate ui;

use self::ui::{InitOptions};
use self::ui::{Menu, MenuItem, Window};

pub fn run() {
    // Options
    const WINDOW_SIZE: (u32, u32) = (640, 480);
    
    // Create menus
    let menu = Menu::new("File");
    menu.append_item("Load Level").on_clicked(Box::new(load_level_clicked));
    menu.append_item("Save Level").on_clicked(Box::new(save_level_clicked));
    
    // Create main window
    let mainwin = Window::new("La bonega karelizisto", WINDOW_SIZE.0 as i32, WINDOW_SIZE.1 as i32, true);
    mainwin.set_margined(true);
    mainwin.on_closing(Box::new(|_| {
        ui::quit();
        false
    }));
    
    // Create toolbox area
    
    
    // Start the application
    mainwin.show();
    ui::main();
}

fn load_level_clicked(_: &MenuItem, mainwin: &Window) {
    match ui::open_file(mainwin) {
        Some(filename) => {
            println!("Opened {:?}", &filename);
        }
        None => {},
    }
}

fn save_level_clicked(_: &MenuItem, mainwin: &Window) {
    match ui::open_file(mainwin) {
        Some(filename) => {
            println!("Saving {:?}", &filename);
        }
        None => {},
    }
}


pub fn start_editor() {
    ui::init(InitOptions).unwrap();
    run();
    ui::uninit();
}
