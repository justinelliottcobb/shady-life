use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Particle {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
}

pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.01, -0.01] },  // Bottom left
    Vertex { position: [ 0.01, -0.01] },  // Bottom right
    Vertex { position: [ 0.01,  0.01] },  // Top right
    Vertex { position: [-0.01,  0.01] },  // Top left
];

pub const INDICES: &[u16] = &[
    0, 1, 2,
    2, 3, 0,
];
