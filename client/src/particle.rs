// particle.rs
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
pub struct ParticleVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl From<&Particle> for ParticleVertex {
    fn from(particle: &Particle) -> Self {
        Self {
            position: particle.position,
            color: particle.color,
        }
    }
}
