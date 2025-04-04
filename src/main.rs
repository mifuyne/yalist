use std::collections::HashMap;

use fltk::{
    app,
    enums::Font,
    group::Group,
    prelude::*,
    text::{StyleTableEntry, TextBuffer, TextDisplay},
    window::Window,
};

mod mainscreen;
mod settings;

struct ChecklistApp {
    lists: HashMap<String, Vec<ListItem>>,
    active_list: String,
}

impl ChecklistApp {
    fn new() -> Self {
        Self {
            lists: HashMap::<String, Vec<ListItem>>::new(),
            active_list: "".to_string(),
        }
    }
}

pub struct ListItem {
    status: bool,
    label: String,
    grp: Group,
}

fn main() {
    let app = app::App::default();
    let mut ui = mainscreen::UserInterface::new("Checklist App");

    app.run().unwrap();
}
