// I want a uniform buffer
// At group 0, binding 0
// And I will treat it as a mat4x4<f32> in the shader. Then the BindGroupLayout says hey the shader is expecting a uniform
// buffer at group 0, binding 0. then the create_bind_group() is saying use this buffre at group 0 bind 0. then the buffer
// holds the acutal data like the camera matrix.

// Camera struct in rust is inited and then the buffer is made then passed into bind_group is like Vertex::desc() where it tells the gpu how to interpret the camera buffer
// the shader then reads from the buffer
struct Camera {
    clip_space: mat4x4<f32>,
    
};

@group(0) @binding(0)
var<uniform> camera: Camera;

// VertexInput: Raw data from your vertex buffer (per-vertex). SO I have the VERTICES struct which includes the 
// vertex points and color and that gets passed in via VertexInput and then the functinos below take and transform that data
// VertexOutput: Transformed data for rendering 

// @location is mapped out in the geometry::Vertex.desc()
struct VertexInput {
    @location(0) c1_position: vec3<f32>, // first attribute is position (x, y, z)
    @location(1) c1_color: vec3<f32>, // second attribute is color (r, g, b)
    // @location(2) c2_position: vec3<f32>,
    // @location(3) c2_color: vec3<f32>, 
};

struct VertexOutput {
    @builtin(position) c1_clip_position: vec4<f32>, // a special value the GPU needs — the final screen position
    @location(0) c1_color: vec3<f32>, 
    //@location(1) c2_color: vec3<f32>, 
};

struct FragOutput {
    @location(0) c1: vec4<f32>,
    //@location(1) c2: vec4<f32>
}

// The @builtin(position) bit tells WGPU that this is the value we want to use as the vertex's clip coordinates (opens new window).
// This means that if your window is 800x600, the x and y of clip_position would be between 0-800 and 0-600, respectively, with the y = 0 being the top of the screen.
// We are using @vertex to mark this function as a valid entry point for a vertex shader. 

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    var proj: ProjectionMatr;

    out.c1_clip_position = vec4<f32>(model.c1_position, 1.0); 
    

    out.c1_color = model.c1_color;
    
    return out;
}
  
@fragment
fn fs_main(in: VertexOutput) -> FragOutput  {
    var f_out: FragOutput;
    f_out.c1 = vec4<f32>(in.c1_color, 1.0);
    
    return f_out;
}
// The @location(0) bit tells WGPU to store the vec4 value returned by this function in the first color target.


