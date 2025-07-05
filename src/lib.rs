use raylib::{color::Color, prelude::RaylibDraw};

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
    }
}
