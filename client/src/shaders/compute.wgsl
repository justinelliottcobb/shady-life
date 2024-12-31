struct Particle {
    position: vec2<f32>,
    velocity: vec2<f32>,
    color: vec4<f32>,
}

struct SimParams {
    delta_time: f32,
    speed: f32,
    boundary_force: f32,
    _padding: f32,
}

@group(0) @binding(0) var<uniform> params: SimParams;
@group(0) @binding(1) var<storage, read_write> particles: array<Particle>;

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&particles)) {
        return;
    }

    var particle = particles[index];

    // Apply velocity
    particle.position += particle.velocity * params.delta_time * params.speed;

    // Boundary handling
    if (particle.position.x < -1.0) {
        particle.velocity.x = abs(particle.velocity.x);
    } else if (particle.position.x > 1.0) {
        particle.velocity.x = -abs(particle.velocity.x);
    }
    if (particle.position.y < -1.0) {
        particle.velocity.y = abs(particle.velocity.y);
    } else if (particle.position.y > 1.0) {
        particle.velocity.y = -abs(particle.velocity.y);
    }

    particles[index] = particle;
}