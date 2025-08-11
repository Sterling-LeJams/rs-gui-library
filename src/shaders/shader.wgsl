
struct Camera {
    cam: mat4x4<f32>,
    projection: mat4x4<f32>,
    
};

@group(0) @binding(0)
var<uniform> camera: Camera;

// @location is mapped out in the geometry::Vertex.desc()
struct VertexInput {
    @location(0) c1_position: vec3<f32>, // first attribute is position (x, y, z)
    @location(1) c1_color: vec3<f32>, // second attribute is color (r, g, b)
};

struct ModelTranslation {
    model: mat4x4<f32>,
};

@group(0) @binding(1)
var<uniform> model_translation: ModelTranslation;

struct VertexOutput {
    @builtin(position) c1_clip_position: vec4<f32>, // a special value the GPU needs — the final screen position
    @location(0) c1_color: vec3<f32>, 
    
};

struct FragOutput {
    @location(0) c1: vec4<f32>,
}

// The @builtin(position) bit tells WGPU that this is the value we want to use as the vertex's clip coordinates (opens new window).
// This means that if your window is 800x600, the x and y of clip_position would be between 0-800 and 0-600, respectively, with the y = 0 being the top of the screen.
// We are using @vertex to mark this function as a valid entry point for a vertex shader. 

@vertex
fn vs_main(model: VertexInput, @builtin(instance_index) instance: u32) -> VertexOutput {
    var out: VertexOutput;

    var model_world_space = vec4<f32>(model.c1_position, 1.0); 

    // Apply translation to instance 1 *before* view/projection
    if (instance == 1u) {
        model_world_space = model_translation.model * model_world_space;
    }

    let view_space = camera.cam * model_world_space;
    out.c1_clip_position = camera.projection * view_space;
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


