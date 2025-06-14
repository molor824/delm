use std::sync::Arc;
use thiserror::Error;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit::error::{EventLoopError, OsError};
use winit::event_loop::ControlFlow;
use crate::renderer;
use crate::widget::Widget;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EventLoop(#[from] EventLoopError),
    #[error(transparent)]
    Os(#[from] OsError),
    #[error(transparent)]
    Renderer(#[from] renderer::Error)
}

pub fn run<T, M, W: Widget<M>>(
    initial: T,
    mut update: impl FnMut(&mut T, M),
    view: impl Fn(&T) -> W,
) -> Result<(), Error> {
    let mut model = initial;
    let widget = view(&model);

    let mut event_loop = EventLoop::new()?;
    let window = Arc::new(WindowBuilder::new().build(&event_loop)?);
    let mut renderer = renderer::Renderer::new(window.clone())?;

    event_loop.set_control_flow(ControlFlow::Wait);

    event_loop
        .run(move |event, window_target| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::RedrawRequested => {
                    renderer.render(wgpu::Color::RED, |_| {}).unwrap();
                }
                event => {
                    window.request_redraw();
                    match event {
                        WindowEvent::Resized(new_size) => renderer.resize(new_size),
                        _ => {}
                    }
                }
            },
            _ => {}
        })
        .map_err(Into::into)
}
