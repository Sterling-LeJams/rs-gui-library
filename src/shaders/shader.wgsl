// Vertex shader
// this shader is getting its data from the vertex buffer

// VertexInput: Raw data from your vertex buffer (per-vertex)
// VertexOutput: Transformed data for rendering

struct VertexInput {
    @location(0) position: vec3<f32>, // first attribute is position (x, y, z)
    @location(1) color: vec3<f32>, // second attribute is color (r, g, b)
    
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, // a special value the GPU needs — the final screen position
    @location(0) color: vec3<f32>, // passes color data from vertex to fragment shader
};

// The @builtin(position) bit tells WGPU that this is the value we want to use as the vertex's clip coordinates (opens new window).
// This means that if your window is 800x600, the x and y of clip_position would be between 0-800 and 0-600, respectively, with the y = 0 being the top of the screen.

// We are using @vertex to mark this function as a valid entry point for a vertex shader. 
@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position =  vec4<f32>(model.position, 1.0); // converts your 3D vertex position into 4D clip space coordinates that the GPU needs for rendering. 1.0 is the w component
    return out;
}
  
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
// The @location(0) bit tells WGPU to store the vec4 value returned by this function in the first color target.


