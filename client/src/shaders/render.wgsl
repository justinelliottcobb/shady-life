// client/src/shaders/render.wgsl

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(input.position, 0.0, 1.0);
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Draw particles as smooth circles
    let center = vec2<f32>(0.5, 0.5);
    let pos = vec2<f32>(input.clip_position.x, input.clip_position.y);
    let dist = distance(pos, center);
    
    // Smoothstep for anti-aliased circle
    let alpha = 1.0 - smoothstep(0.45, 0.5, dist);
    return vec4<f32>(input.color.rgb, input.color.a * alpha);
}