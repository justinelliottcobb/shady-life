// client/src/shaders/mod.rs

// We include our shader code directly in our binary
// This makes deployment simpler while still keeping shaders in separate files
pub const COMPUTE_SHADER: &str = include_str!("compute.wgsl");
pub const RENDER_SHADER: &str = include_str!("render.wgsl");

use wgpu::ShaderModule;

pub struct Shaders {
    pub compute: ShaderModule,
    pub render: ShaderModule,
}

impl Shaders {
    // Create both shader modules at once
    pub fn new(device: &wgpu::Device) -> Self {
        let compute = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(COMPUTE_SHADER.into()),
        });

        let render = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Render Shader"),
            source: wgpu::ShaderSource::Wgsl(RENDER_SHADER.into()),
        });

        Self { compute, render }
    }
}

// Helper function to create a compute pipeline using our compute shader
pub fn create_compute_pipeline(
    device: &wgpu::Device,
    shader_module: &ShaderModule,
    layout: &wgpu::PipelineLayout,
) -> wgpu::ComputePipeline {
    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Particle Compute Pipeline"),
        layout: Some(layout),
        module: shader_module,
        entry_point: "main",
    })
}

// Helper function to create a render pipeline using our render shader
pub fn create_render_pipeline(
    device: &wgpu::Device,
    shader_module: &ShaderModule,
    layout: &wgpu::PipelineLayout,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Particle Render Pipeline"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader_module,
            entry_point: "vs_main",
            buffers: &[
                // We'll define our vertex buffer layout here later
            ],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader_module,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::PointList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    })
}
