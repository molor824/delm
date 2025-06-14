use pollster::FutureExt;
use std::sync::Arc;
use thiserror::Error;
use wgpu::*;
use winit::dpi::PhysicalSize;
use winit::window::Window;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CreateSurface(#[from] CreateSurfaceError),
    #[error(transparent)]
    RequestAdapter(#[from] RequestAdapterError),
    #[error(transparent)]
    RequestDevice(#[from] RequestDeviceError),
    #[error(transparent)]
    Surface(#[from] SurfaceError),
}
pub struct Renderer {
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    device: Device,
    queue: Queue,
    adapter: Adapter,
    instance: Instance,
}
impl Renderer {
    pub fn new(window: Arc<Window>) -> Result<Self, Error> {
        async {
            let instance = Instance::new(&InstanceDescriptor {
                backends: Backends::PRIMARY,
                ..Default::default()
            });
            let surface = instance.create_surface(window.clone())?;
            let adapter = instance
                .request_adapter(&RequestAdapterOptions {
                    compatible_surface: Some(&surface),
                    ..Default::default()
                })
                .await?;
            let (device, queue) = adapter
                .request_device(&DeviceDescriptor {
                    ..Default::default()
                })
                .await?;
            let surface_caps = surface.get_capabilities(&adapter);
            let size = window.inner_size();
            let surface_config = SurfaceConfiguration {
                width: size.width,
                height: size.height,
                format: surface_caps
                    .formats
                    .iter()
                    .find(|f| f.is_srgb())
                    .copied()
                    .unwrap_or(surface_caps.formats[0]),
                usage: TextureUsages::RENDER_ATTACHMENT,
                present_mode: surface_caps.present_modes[0],
                alpha_mode: surface_caps.alpha_modes[0],
                desired_maximum_frame_latency: 2,
                view_formats: vec![],
            };
            surface.configure(&device, &surface_config);

            Ok(Self {
                instance,
                surface,
                surface_config,
                device,
                queue,
                adapter,
            })
        }
        .block_on()
    }
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }
    pub fn render(
        &self,
        clear_color: Color,
        on_render_pass: impl FnOnce(&mut RenderPass),
    ) -> Result<(), Error> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(clear_color),
                        store: StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            on_render_pass(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
