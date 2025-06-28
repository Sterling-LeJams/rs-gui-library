// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};
// The @builtin(position) bit tells WGPU that this is the value we want to use as the vertex's clip coordinates (opens new window).
// This means that if your window is 800x600, the x and y of clip_position would be between 0-800 and 0-600, respectively, with the y = 0 being the top of the screen.

// We are using @vertex to mark this function as a valid entry point for a vertex shader. 
@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}
 
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}
// The @location(0) bit tells WGPU to store the vec4 value returned by this function in the first color target.


