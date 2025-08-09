
use nalgebra::Translation3;


// this is a vertex buffer so the shader is not hard coded and will not have to recompile everytime you want to change it.

// you are creating a struct that has the position and color of the vertex. But the GPU has no idea what that means.
//  so you have to create a vertex buffer description so it knows what those bytes means otherwise it just sees this:
// |    12 bytes    |     12 bytes     |
// | [f32; 3] pos   | [f32; 3] color   |

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    // tells gpu how to interpret the buffer
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0, // this maps to the @location(0) in WGSL
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}


#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct Cube {
    pub vertices: [Vertex; 4],
}
impl Cube {
    pub fn new() -> Self {
        Self {
            vertices: [
                Vertex { position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 1.0] }, 
                Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 1.0, 1.0] }, 
                Vertex { position: [0.5, 0.5, 0.0], color: [1.0, 1.0, 1.0] }, 
                Vertex { position: [-0.5, 0.5, 0.0], color: [0.0, 0.0, 0.0] },
            ],
        }
    }

    pub fn to_world_space(&self) -> Self {
        let translation = Translation3::new(-0.5, 0.8, 0.0);

        // Convert to a 4x4 matrix
        let translated_vertices = self.vertices.map(|each_vertices| {
            let homogeonous_vert = nalgebra::Vector4::new(each_vertices.position[0],each_vertices.position[1],each_vertices.position[2],1.0,);
            let world_space = translation.to_homogeneous() * homogeonous_vert;
            let world_space_mat = world_space.xyz(); //drop w component 

            Vertex {
                position: world_space_mat.into(), 
                color: each_vertices.color
            }
        });

        Self {
            vertices: translated_vertices,
        }
    }
}

// to draw triangles on each square face
pub const INDICES: [u16; 12] = [
       // Rectangle 1
    0, 1, 2,
    2, 3, 0,
    // Rectangle 2 (offset by 4)
    4, 5, 6,
    6, 7, 4,
];

// should probably have some sort of erro handeling ot match the vertices with the indices



