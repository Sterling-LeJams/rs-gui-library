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
    
    // Simple 3D transformation with rotation
    let time = 0.0; // You can pass this as a uniform for animation
    let rotation_y = time;
    let rotation_x = time * 0.5;
    
    // Apply rotation matrices
    let cos_y = cos(rotation_y);
    let sin_y = sin(rotation_y);
    let cos_x = cos(rotation_x);
    let sin_x = sin(rotation_x);
    
    // Rotate around Y axis
    var pos = model.position;
    let rotated_y = vec3<f32>(
        pos.x * cos_y - pos.z * sin_y,
        pos.y,
        pos.x * sin_y + pos.z * cos_y
    );
    
    // Rotate around X axis
    let rotated_x = vec3<f32>(
        rotated_y.x,
        rotated_y.y * cos_x - rotated_y.z * sin_x,
        rotated_y.y * sin_x + rotated_y.z * cos_x
    );
    
    // Move cube back a bit so we can see it
    let final_pos = vec3<f32>(rotated_x.x, rotated_x.y, rotated_x.z - 2.0);
    
    // Simple perspective projection
    let aspect = 800.0 / 600.0; // Adjust to your window size
    let fov = 45.0 * 3.14159 / 180.0; // 45 degrees in radians
    let near = 0.1;
    let far = 100.0;
    
    let f = 1.0 / tan(fov * 0.5);
    
    out.clip_position = vec4<f32>(
        final_pos.x * f / aspect,
        final_pos.y * f,
        final_pos.z * (far + near) / (near - far) + (2.0 * far * near) / (near - far),
        -final_pos.z
    ); // converts your 3D vertex position into 4D clip space coordinates that the GPU needs for rendering. 1.0 is the w component
    return out;
}
  
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
// The @location(0) bit tells WGPU to store the vec4 value returned by this function in the first color target.


