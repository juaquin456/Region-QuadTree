use std::ascii::escape_default;
use std::ops::Deref;
use wgpu::{InstanceDescriptor, StoreOp};
use wgpu::TextureFormat::Bgra8Unorm;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use tokio::runtime::Runtime;
use winit::dpi::Size::Physical;
use winit::error::ExternalError;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    color: wgpu::Color,

    window: winit::window::Window,
}

impl State {
    async fn new(window: Window) -> Self {
        let clear_color = wgpu::Color {
            r:0.6,
            g:0.2,
            b:0.1,
            a:1.0,
        };

        let size = window.inner_size();
        let mut descriptor = wgpu::InstanceDescriptor::default();
        descriptor.backends = wgpu::Backends::all();
        let instance = wgpu::Instance::new(descriptor);
        let surface = unsafe { instance.create_surface(&window).unwrap() };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),

                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await
            .unwrap();
        dbg!(adapter.get_info());
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: Bgra8Unorm,
            width: size.width,
            height: size.height,
            present_mode: Default::default(),
            alpha_mode: Default::default(),
            view_formats: vec![],
        };

        surface.configure(&device, &config);
        Self {
            surface,
            device,
            queue,
            config,
            size,
            color: clear_color,
            window,
        }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config)
        }
    }
    fn rescale(&mut self, factor: f64) {
        let new_size = PhysicalSize::new(
            self.size.width * factor as u32,
            self.size.height * factor as u32,
        );
        self.resize(new_size);
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved {device_id, position} => {
                self.color.r = position.x / self.size.width as f64;
                self.color.g = position.y / self.size.height as f64;
                self.window.request_redraw();
                true
            }
            _ => {false}
        }
    }

    fn update(&mut self) {
        // todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(
            &wgpu::TextureViewDescriptor::default(),
        );
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );

        {
            let _render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(
                                    self.color,
                                ),
                                store: StoreOp::Store,
                            },
                        },
                    )],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                },
            );
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("QuadTree")
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(window).await;

    event_loop
        .run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window.id() => if !state.input(event) {
                    match event {
                        WindowEvent::ScaleFactorChanged {scale_factor, ..} => {
                            state.rescale(*scale_factor);
                        }
                        WindowEvent::RedrawRequested{} => {
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                // Reconfigure the surface if lost
                                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                                // All other errors (Outdated, Timeout) should be resolved by the next frame
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        WindowEvent::Resized(new_size) => {
                            state.resize(*new_size);
                        }
                        WindowEvent::CloseRequested => {
                            println!("The close button was pressed; stopping");
                            elwt.exit();
                        }
                        _=>{}
                    }
                }
                Event::AboutToWait => {
                    // Application update code.

                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw, in
                    // applications which do not always need to. Applications that redraw continuously
                    // can just render here instead.
                    // state.window.request_redraw();
                }
                _ => (),
            }
        })
        .expect("TODO: panic message");
}

fn main() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(run());
}
