use raylib::{color::Color, math::Vector2, prelude::RaylibDraw};

use crate::renderer::{Corner, Renderer};

pub mod renderer;
pub mod widget;
pub mod window;

pub fn render<Model, Msg>(
    mut model: Model,
    mut window: impl FnMut(&Model) -> window::Builder,
    mut render: impl FnMut(&Model),
    mut update: impl FnMut(&mut Model, Msg),
) {
    let mut window = window(&model).build().unwrap();
    let (mut rl, thread) = raylib::init()
        .vsync()
        .size(window.size.0 as i32, window.size.1 as i32)
        .title(&window.title)
        .build();

    while !rl.window_should_close() {
        let mut handle = rl.begin_drawing(&thread);
        handle.clear_background(Color::new(0, 0, 0, 255));
        handle.draw_text("Hello!", 10, 10, 20, Color::new(200, 200, 200, 255));

        let mut renderer = Renderer::from_handle(handle);
        renderer.draw_rectangle(
            Vector2::new(20.0, 30.0),
            Vector2::new(200.0, 150.0),
            Corner::new(20.0, 10.0, 0.0, 40.0),
            Color::RED,
        );
    }
}
