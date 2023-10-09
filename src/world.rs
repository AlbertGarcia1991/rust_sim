use std::iter;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// A structure to hold the displayed window and all its content.
struct State {
    /// A `Surface` represents a platform-specific surface (e.g. a window) onto which rendered 
    /// images may be presented.
    surface: wgpu::Surface,
    /// Open connection to a graphics and/or compute device. Responsible for the creation of most 
    /// rendering and compute resources.
    device: wgpu::Device,
    /// Handle to a command queue on a device.
    queue: wgpu::Queue,
    /// Described the Surface's canvas. This is the part of the window that we draw to.
    config: wgpu::SurfaceConfiguration,
    /// A structure representing the window size. It has width and height as attributes.
    size: winit::dpi::PhysicalSize<u32>,
    /// WGPU window object. Needs to be declared after the surface so it gets dropped after it as 
    /// the surface contains unsafe references to the window's resources. 
    window: Window,
    clear_color: wgpu::Color,
}

impl State {
    /// A trait to create a new window
    async fn new(window: Window) -> Self {
        let size: winit::dpi::PhysicalSize<u32> = window.inner_size();

        // The instance is a handle to our GPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // The surface needs to live as long as the window that created it. State owns the window so 
        // this should be safe
        let surface: wgpu::Surface = unsafe { 
            instance.create_surface(&window)
        }.unwrap();

        // Get properties specific for current machine and OS in order to create the window later
        let adapter: wgpu::Adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        
        // Here we are defining how our window is going to be related to our hardware (GPU)
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    // features allows us to specify what extra features we want. This is GPU
                    // dependent and you can check the features available via adapter.features()
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if we're building for the 
                    // web we qill have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },

                None,
            )
            .await
            .unwrap();
        
        // Here we are defining a config for our surface.
        let surface_caps: wgpu::SurfaceCapabilities = surface.get_capabilities(&adapter);

        // Defines how the surface creates its underlying SurfaceTexture object
        let surface_format: wgpu::TextureFormat = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f: &wgpu::TextureFormat| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        
        // The config defines 
        let config = wgpu::SurfaceConfiguration {
            // Defines how SurfaceTexture will be used. RENDER_ATTACHMENT defines that the textures
            // will be used to write to the screen
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,  
            // Defines how the SurfaceTexture will be stored on the GPU
            format: surface_format,
            // Width and Height in pixels of the SurfaceTexture. ! Ensure none is zero since this 
            // may lead to GPU crash
            width: size.width,
            height: size.height,
            // Defines for to sync the surface with the display. We select here VSync which is 
            // always supported
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            // List of TextureFormat that you can use when creating TextureView
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // let clear_color: wgpu::Color = wgpu::Color::BLACK;
        let clear_color: wgpu::Color = wgpu::Color {
            r: 0.9,
            g: 0.9,
            b: 0.9,
            a: 0.9,
        };

        // Set State struct attributes
        Self {
            surface: surface,
            device: device,
            queue: queue,
            config: config,
            clear_color: clear_color,
            size: size,
            window: window,
        }
    }

    fn window(&self) -> &Window { &self.window }

    /// A trait to support resizing the window. This is called every time the window size changes.
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            // Reconfigure Window and Surface with new dimensions
            self.surface.configure(&self.device, &self.config);
        }
    }

    #[allow(unused_variables)]
    /// Returns a bool to indicate whether an event has been fully processed. If this method returns
    /// false, the main loop will not process the event any further.
    fn input(&mut self, event: &WindowEvent) -> bool {
        // match event {
        //     WindowEvent::CursorMoved { position, .. } => {
        //         self.clear_color = wgpu::Color {
        //             r: position.x as f64 / self.size.width as f64,
        //             g: position.y as f64 / self.size.height as f64,
        //             b: 1.0,
        //             a: 1.0,
        //         };
        //         true
        //     }
        //     _ => false,
        // }
        false
    }

    fn update(&mut self) {}

    /// This is the method that draws the content on our Surface
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // We get the current frame Surface
        let output: wgpu::SurfaceTexture = self.surface.get_current_texture()?;
        // We create a TextureView with default settings
        let view: wgpu::TextureView = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        // We also need to create a CommandEncoder to create the actual commands to send to the gpu.
        // Most modern graphics frameworks expect commands to be stored in a command buffer before 
        // being sent to the gpu. The encoder builds a command buffer that we send to the gpu
        let mut encoder: wgpu::CommandEncoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        
        // This is the block that will erase the current Surface and draw the new one
        {
            let _render_pass: wgpu::RenderPass<'_> = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    // Describer where we are going to draw our color to
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            // We clear the previous frame before drawing new one
                            load: wgpu::LoadOp::Clear(self.clear_color),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                }
            );
        }
        // We submit the new Surface to the screen. This needs to happen once we have released the 
        // mutable encoder borrowed on the previous block
        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    // Set configuration for the specific platform (OS or Web)
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop: EventLoop<()> = EventLoop::new();
    let window: Window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    // State::new uses async code, so we're going to wait for it to finish
    let mut state: State = State::new(window).await;

    event_loop.run(move |event: Event<'_, ()>, _, control_flow: &mut ControlFlow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        // Case closing window
                        WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        // Case manual resize
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        // Case forced resize with events such as display resolution changed
                        WindowEvent::ScaleFactorChanged {
                            new_inner_size, ..
                        } => {
                                // new_inner_size is &&mut so w have to dereference it twice
                                state.resize(**new_inner_size);
                            }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually request it.
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}