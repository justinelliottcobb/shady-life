struct Particle {
    position: vec2<f32>,
    velocity: vec2<f32>,
    particle_type: u32,
    padding: u32,
};

struct SimParams {
    deltaTime: f32,
    rule_matrix: array<array<f32, 4>, 4>, // 4x4 matrix for particle type interactions
    repulsion_radius: f32,
    attraction_radius: f32,
};

@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;
@group(0) @binding(1) var<uniform> params: SimParams;

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&particles)) {
        return;
    }

    var particle = particles[index];
    var force = vec2<f32>(0.0, 0.0);

    // Calculate forces from all other particles
    for (var i = 0u; i < arrayLength(&particles); i = i + 1u) {
        if (i == index) {
            continue;
        }

        let other = particles[i];
        let diff = other.position - particle.position;
        let dist_sq = dot(diff, diff);
        let dist = sqrt(dist_sq);

        if (dist < params.repulsion_radius) {
            // Repulsion force
            force = force - normalize(diff) * (1.0 - dist / params.repulsion_radius);
        } else if (dist < params.attraction_radius) {
            // Attraction force based on particle types
            let force_multiplier = params.rule_matrix[particle.particle_type][other.particle_type];
            force = force + normalize(diff) * force_multiplier * (dist - params.repulsion_radius) 
                / (params.attraction_radius - params.repulsion_radius);
        }
    }

    // Update velocity and position
    particle.velocity = particle.velocity + force * params.deltaTime;
    particle.position = particle.position + particle.velocity * params.deltaTime;

    // Boundary conditions (wrap around)
    if (particle.position.x < -1.0) { particle.position.x = 1.0; }
    if (particle.position.x > 1.0) { particle.position.x = -1.0; }
    if (particle.position.y < -1.0) { particle.position.y = 1.0; }
    if (particle.position.y > 1.0) { particle.position.y = -1.0; }

    particles[index] = particle;
}
