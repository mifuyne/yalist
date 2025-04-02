use fltk::{self, group::Group};

#[derive(Debug)]
pub struct Settings {
    grp: Group,
    test_option: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            grp: Group::default(),
            test_option: false,
        }
    }
}
