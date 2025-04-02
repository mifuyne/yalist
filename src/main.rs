use std::collections::HashMap;

use fltk::{
    app,
    enums::Font,
    prelude::*,
    text::{StyleTableEntry, TextBuffer, TextDisplay},
    window::Window,
};

mod settings;

struct ChecklistApp {
    lists: HashMap<String, Vec<ListItem>>,
    active_list: String,
}

struct ListItem {
    status: bool,
    label: String,
}

fn main() {
    let app = app::App::default().load_system_fonts();
    let fonts = app::fonts();
    println!("{:?}", fonts);
    let mut win = Window::new(100, 100, 400, 300, "Hello from fltk-rs");
    let settings = settings::Settings::new();

    println!("{:?}", settings);

    app.run().unwrap();
}
