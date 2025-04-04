use fltk::{
    app,
    button::{self, Button},
    enums::{Color, FrameType},
    frame::Frame,
    group::{Flex, Scroll, ScrollType},
    input::Input,
    prelude::*,
    window::Window,
};

pub struct UserInterface {
    pub win: Window,
    pub list_view: Flex,
    pub input: Input,
}

impl UserInterface {
    pub fn new(title: &str) -> Self {
        let scrn_dimensions = app::screen_xywh(0);
        let mut win = Window::new(
            scrn_dimensions.2 / 2,
            scrn_dimensions.3 / 2,
            320,
            600,
            title,
        );
        win.make_resizable(true);
        let mut container = Flex::default_fill().column();
        container.end();
        win.end();
        win.show();

        // Testing Tile Group

        let mut list_scroll = Scroll::new(0, 0, container.width(), container.height() - 50, "")
            .with_type(ScrollType::Vertical);
        list_scroll.set_frame(FrameType::FlatBox);
        list_scroll.set_color(Color::Red);

        let mut list_view = Flex::default().with_size(win.width(), 50).column();
        list_view.set_frame(FrameType::FlatBox);
        list_view.set_color(Color::Green);
        list_view.make_resizable(true);

        list_scroll.end();

        // list_view.layout();

        let input = Input::new(0, container.height() - 50, container.width(), 50, "");

        list_scroll.add(&list_view);
        container.add(&list_scroll);
        container.add(&input);
        container.fixed(&input, 50);

        Self {
            win,
            list_view,
            input,
        }
    }
}
