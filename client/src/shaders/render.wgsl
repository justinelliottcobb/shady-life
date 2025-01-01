struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct InstanceInput {
    @location(1) particle_pos: vec2<f32>,
    @location(2) particle_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var output: VertexOutput;
    // Transform the vertex position by the particle position
    let world_pos = vertex.position + instance.particle_pos;
    output.clip_position = vec4<f32>(world_pos, 0.0, 1.0);
    output.color = instance.particle_color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}