mod particle;
mod simulation;
mod render;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use std::sync::Arc;
use rand::Rng;
use wgpu::util::DeviceExt;

use particle::{Particle, ParticleVertex};
use simulation::{ComputePass, SimParams};
use render::RenderPass;

const INITIAL_NUM_PARTICLES: u32 = 1000;
const WORKGROUP_SIZE: u32 = 256;

#[wasm_bindgen]
pub struct ParticleLife {
    surface: wgpu::Surface<'static>,
    device: Arc<wgpu::Device>,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (u32, u32),
    render_pass: RenderPass,
    vertex_buffer: wgpu::Buffer,
    compute_pass: ComputePass,
    particle_buffer: wgpu::Buffer,
    num_particles: u32,
    last_time: f64,
}

#[wasm_bindgen]
impl ParticleLife {
    #[wasm_bindgen(constructor)]
    pub async fn new(canvas: HtmlCanvasElement) -> Result<ParticleLife, JsValue> {
        let size = (canvas.width(), canvas.height());
        
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
            .map_err(|e| JsValue::from_str(&format!("Failed to create surface: {}", e)))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or_else(|| JsValue::from_str("No suitable GPU adapter found"))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor { label: None, required_features: wgpu::Features::empty(), required_limits: wgpu::Limits::default() },
                None,
            )
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to create device: {}", e)))?;
        let device = Arc::new(device);

        let config = init_surface_config(&surface, &adapter, size);
        surface.configure(&device, &config);

        let particles = init_particles();
        let (particle_buffer, vertex_buffer) = create_buffers(&device, &particles);

        let render_pass = RenderPass::new(&device, config.format);
        let compute_pass = ComputePass::new(&device, &particle_buffer);

        Ok(ParticleLife {
            surface,
            device,
            queue,
            config,
            size,
            render_pass,
            vertex_buffer,
            compute_pass,
            particle_buffer,
            num_particles: INITIAL_NUM_PARTICLES,
            last_time: 0.0,
        })
    }

    #[wasm_bindgen]
    pub fn render(&mut self) -> Result<(), JsValue> {
        let dt = self.calculate_delta_time()?;
        self.update_simulation(dt);
    
        let frame = self.get_next_frame()?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Command Encoder"),
        });
    
        self.execute_compute_pass(&mut encoder);
    
        // Add copy command
        let mut copy_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Copy Buffer Encoder"),
        });
    
        copy_encoder.copy_buffer_to_buffer(
            &self.particle_buffer,
            0,
            &self.vertex_buffer,
            0,
            (self.num_particles * std::mem::size_of::<ParticleVertex>() as u32) as u64,
        );
    
        self.queue.submit(vec![encoder.finish(), copy_encoder.finish()]);
    
        self.execute_render_pass(&mut self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        }), &view);
    
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

    fn calculate_delta_time(&mut self) -> Result<f32, JsValue> {
        let now = web_sys::window()
            .unwrap()
            .performance()
            .unwrap()
            .now() / 1000.0;
        
        let dt = if self.last_time == 0.0 {
            1.0 / 60.0
        } else {
            (now - self.last_time) as f32
        };
        self.last_time = now;
        Ok(dt)
    }

    fn update_simulation(&mut self, dt: f32) {
        let sim_params = SimParams {
            delta_time: dt,
            speed: 1.0,
            boundary_force: 1.0,
            _padding: 0.0,
        };
        self.queue.write_buffer(&self.compute_pass.params_buffer, 0, bytemuck::cast_slice(&[sim_params]));
    }

    fn get_next_frame(&self) -> Result<wgpu::SurfaceTexture, JsValue> {
        self.surface
            .get_current_texture()
            .map_err(|e| JsValue::from_str(&format!("Failed to get next frame: {}", e)))
    }

    fn execute_compute_pass(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&self.compute_pass.pipeline);
        compute_pass.set_bind_group(0, &self.compute_pass.bind_group, &[]);
        compute_pass.dispatch_workgroups((self.num_particles + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE, 1, 1);
    }

    fn execute_render_pass(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.1, b: 0.1, a: 1.0 }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.render_pass.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.num_particles, 0..1);
    }
}

fn init_surface_config(
    surface: &wgpu::Surface,
    adapter: &wgpu::Adapter,
    size: (u32, u32),
) -> wgpu::SurfaceConfiguration {
    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats.iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.0,
        height: size.1,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    }
}

fn init_particles() -> Vec<Particle> {
    let mut particles = Vec::with_capacity(INITIAL_NUM_PARTICLES as usize);
    let mut rng = rand::thread_rng();
    
    for _ in 0..INITIAL_NUM_PARTICLES {
        particles.push(Particle {
            position: [rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)],
            velocity: [rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1)],
            color: [rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0), 1.0],
        });
    }
    particles
}

fn create_buffers(
    device: &wgpu::Device,
    particles: &[Particle],
) -> (wgpu::Buffer, wgpu::Buffer) {
    let particle_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Particle Buffer"),
        contents: bytemuck::cast_slice(particles),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
    });

    let vertices: Vec<ParticleVertex> = particles.iter().map(|p| p.into()).collect();
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    });

    (particle_buffer, vertex_buffer)
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    if let Err(e) = console_log::init_with_level(log::Level::Info) {
        web_sys::console::error_1(&format!("Failed to initialize logging: {:?}", e).into());
    } else {
        log::info!("WebAssembly module initialized with logging enabled");
    }
}
