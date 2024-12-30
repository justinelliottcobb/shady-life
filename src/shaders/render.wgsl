// client/src/shaders/render.wgsl

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) particle_type: u32,
};

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) velocity: vec2<f32>,
    @location(2) particle_type: u32,
) -> VertexOutput {
    var output: VertexOutput;
    
    // Convert from simulation space (-1 to 1) to clip space
    output.position = vec4<f32>(position, 0.0, 1.0);
    output.particle_type = particle_type;
    
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Different colors for different particle types
    let colors = array<vec3<f32>, 4>(
        vec3<f32>(1.0, 0.2, 0.2),  // Red
        vec3<f32>(0.2, 1.0, 0.2),  // Green
        vec3<f32>(0.2, 0.2, 1.0),  // Blue
        vec3<f32>(1.0, 1.0, 0.2)   // Yellow
    );
    
    // Get color based on particle type
    let color = colors[in.particle_type];
    
    // Add some basic shading based on position
    return vec4<f32>(color, 1.0);
}
