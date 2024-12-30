use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use std::sync::Arc;

// This struct represents our particle simulation state
#[wasm_bindgen]
pub struct ParticleLife {
    surface: wgpu::Surface<'static>,
    device: Arc<wgpu::Device>,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (u32, u32),
}

#[wasm_bindgen]
impl ParticleLife {
    #[wasm_bindgen(constructor)]
    pub async fn new(canvas: HtmlCanvasElement) -> Result<ParticleLife, JsValue> {
        log::info!("Initializing ParticleLife simulation");

        // Get the size from the canvas
        let size = (canvas.width(), canvas.height());

        // Create the wgpu instance with WebGL fallback
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
            ..Default::default()
        });

        // Create the surface using the canvas
        let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
            .map_err(|e| JsValue::from_str(&format!("Failed to create surface: {}", e)))?;

        // Request adapter with WebGPU support
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or_else(|| JsValue::from_str("No suitable GPU adapter found"))?;

        // Create device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to create device: {}", e)))?;
        let device = Arc::new(device);

        // Configure the surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        
        surface.configure(&device, &config);

        log::info!("ParticleLife initialization complete");

        Ok(ParticleLife {
            surface,
            device,
            queue,
            config,
            size,
        })
    }

    #[wasm_bindgen]
    pub fn render(&self) -> Result<(), JsValue> {
        let frame = self.surface
            .get_current_texture()
            .map_err(|e| JsValue::from_str(&format!("Failed to get next frame: {}", e)))?;

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // Create a render pass to clear the screen to a dark color
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.size = (width, height);
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}

// Initialize panic hook and logging
#[wasm_bindgen(start)]
pub fn start() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Initialize logging exactly once
    if let Err(e) = console_log::init_with_level(log::Level::Info) {
        // If logging fails, report it to the console directly
        web_sys::console::error_1(&format!("Failed to initialize logging: {:?}", e).into());
    } else {
        log::info!("WebAssembly module initialized with logging enabled");
    }
}
