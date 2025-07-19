
struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};

// I want a uniform buffer
// At group 0, binding 0
// And I will treat it as a mat4x4<f32> in the shader. Then the BindGroupLayout says hey the shader is expecting a uniform
// buffer at group 0, binding 0. then the create_bind_group() is saying use this buffre at group 0 bind 0. then the buffer
// holds the acutal data like the camera matrix.
@group(0) @binding(0)
var<uniform> camera: Camera;


// VertexInput: Raw data from your vertex buffer (per-vertex)
// VertexOutput: Transformed data for rendering

struct VertexInput {
    @location(0) position: vec3<f32>, // first attribute is position (x, y, z)
    @location(1) color: vec3<f32>, // second attribute is color (r, g, b)
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, // a special value the GPU needs — the final screen position
    @location(0) color: vec3<f32>, // passes color data from vertex to fragment shader
    @location(2) world_position: vec3<f32>,
    @location(3) camera_view_pos: vec4<f32>,
};

// view proj matrices to apply to vertices to bring it into inside clipping space 
// i do not think i need the locatino(0)
struct ProjectionMatr {
    @location(0) proj_mat: mat4x4<f32>,
};

// The @builtin(position) bit tells WGPU that this is the value we want to use as the vertex's clip coordinates (opens new window).
// This means that if your window is 800x600, the x and y of clip_position would be between 0-800 and 0-600, respectively, with the y = 0 being the top of the screen.

// We are using @vertex to mark this function as a valid entry point for a vertex shader. 
@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
  
    var world_position: vec4<f32> = vec4<f32>(model.position, 1.0);
    out.world_position = world_position.xyz;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.camera_view_pos = camera.view_pos;
    return out;
}
  
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
// The @location(0) bit tells WGPU to store the vec4 value returned by this function in the first color target.


