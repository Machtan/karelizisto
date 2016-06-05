extern crate libui;

use self::ui::{BoxControl, Button, Checkbox, ColorButton, Combobox, DateTimePicker, EditableCombobox};
use self::ui::{Entry, FontButton, Group, InitOptions, Label, Menu, MenuItem, ProgressBar, RadioButtons};
use self::ui::{Separator, Slider, Spinbox, Tab, Window};

pub fn run() {
    ui::init(InitOptions).unwrap();
    
    let menu = Menu::new("File");
    menu.append_item("Open").on_clicked(Box::new(open_clicked));
    menu.append_item("Save").on_clicked(Box::new(save_clicked));

    let menu = Menu::new("Edit");
    menu.append_check_item("Checkable Item");
    menu.append_separator();
    let item = menu.append_item("Disabled Item");
    item.disable();
    menu.append_preferences_item();

    let menu = Menu::new("Help");
    menu.append_item("Help");
    menu.append_about_item();

    let mainwin = Window::new("ui Control Gallery", 640, 480, true);
    mainwin.set_margined(true);
    mainwin.on_closing(Box::new(|_| {
        ui::quit();
        false
    }));

    let vbox = BoxControl::new_vertical();
    vbox.set_padded(true);
    mainwin.set_child(vbox.clone());

    let hbox = BoxControl::new_horizontal();
    hbox.set_padded(true);
    vbox.append(hbox.clone(), true);

    let group = Group::new("Basic Controls");
    group.set_margined(true);
    hbox.append(group.clone(), false);

    let inner = BoxControl::new_vertical();
    inner.set_padded(true);
    group.set_child(inner.clone());

    inner.append(Button::new("Button"), false);
    inner.append(Checkbox::new("Checkbox"), false);
    let entry = Entry::new();
    entry.set_text("Entry");
    inner.append(entry, false);
    inner.append(Label::new("Label"), false);
    inner.append(Separator::new_horizontal(), false);

    inner.append(DateTimePicker::new_date_picker(), false);
    inner.append(DateTimePicker::new_time_picker(), false);
    inner.append(DateTimePicker::new_date_time_picker(), false);

    inner.append(FontButton::new(), false);
    inner.append(ColorButton::new(), false);

    let inner2 = BoxControl::new_vertical();
    inner2.set_padded(true);
    hbox.append(inner2.clone(), true);

    let group = Group::new("Numbers");
    group.set_margined(true);
    inner2.append(group.clone(), false);

    let inner = BoxControl::new_vertical();
    inner.set_padded(true);
    group.set_child(inner.clone());

    let spinbox = Spinbox::new(0, 100);
    spinbox.on_changed(Box::new(|spinbox| update(spinbox.value())));
    inner.append(spinbox, false);

    let slider = Slider::new(0, 100);
    slider.on_changed(Box::new(|slider| update(slider.value())));
    inner.append(slider, false);

    let progress_bar = ProgressBar::new();
    inner.append(progress_bar, false);

    let group = Group::new("Lists");
    group.set_margined(true);
    inner2.append(group.clone(), false);

    let inner = BoxControl::new_vertical();
    inner.set_padded(true);
    group.set_child(inner.clone());

    let cbox = Combobox::new();
    cbox.append("Combobox Item 1");
    cbox.append("Combobox Item 2");
    cbox.append("Combobox Item 3");
    inner.append(cbox, false);

    let ecbox = EditableCombobox::new();
    ecbox.append("Editable Item 1");
    ecbox.append("Editable Item 2");
    ecbox.append("Editable Item 3");
    inner.append(ecbox, false);

    let rb = RadioButtons::new();
    rb.append("Radio Button 1");
    rb.append("Radio Button 2");
    rb.append("Radio Button 3");
    inner.append(rb, true);

    let tab = Tab::new();
    tab.append("Page 1", BoxControl::new_horizontal());
    tab.append("Page 2", BoxControl::new_horizontal());
    tab.append("Page 3", BoxControl::new_horizontal());
    inner2.append(tab, true);

    mainwin.show();
    ui::main();
    
    ui::uninit();
}

fn open_clicked(_: &MenuItem, mainwin: &Window) {
    match ui::open_file(mainwin) {
        Some(filename) => ui::msg_box(mainwin, "File selected", &*filename),
        None => ui::msg_box_error(mainwin, "No file selected", "Don't be alarmed!"),
    }
}

fn save_clicked(_: &MenuItem, mainwin: &Window) {
    match ui::open_file(mainwin) {
        Some(filename) => {
            ui::msg_box(mainwin, "File selected (don't worry, it's still there)", &*filename)
        }
        None => ui::msg_box_error(mainwin, "No file selected", "Don't be alarmed!"),
    }
}

fn update(_: i64) {
    // TODO(pcwalton)
}