use anyhow::Result;
use nalgebra::Translation3;
use nalgebra::base::Matrix4;
use nalgebra::geometry;
use std::{num::NonZeroU64};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry,
    util::{DeviceExt},
};

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

// X and Y should be between -1 and 1
// Z should be between 0 and 1 (or -1 and 1 depending on your setup)
// Not a cube yet just the first face
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
struct Cube {
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

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub cam_mat: [[f32; 4]; 4],
    pub clip_space: [[f32; 4]; 4], // mat4x4<f32>
}
// CURRENTLY THIS ONLY TAKING ONE MATRIX AND I AM PASSING IN CameraMatrix::new().to_clip_space(); but I do not know that I should be passing this in.
// WHAT IF I WANT THE JUST CAMERA POSITION AS A BUFFER
impl From<CameraMatrix> for CameraUniform {
    fn from(cam: CameraMatrix) -> Self {
        // I THINK THE GOAL IS TO MAKE THE FIRST 64 BYTES ASSIGNED TO viewProj, then the next assign to cam
        let cam_mat: [[f32; 4]; 4] = cam
            .cam_matrix
            .as_slice()
            .chunks(4)
            .map(|chunk| <[f32; 4]>::try_from(chunk).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let clip_space: [[f32; 4]; 4] = cam
            .clip_space
            .as_slice()
            .chunks(4)
            .map(|chunk| <[f32; 4]>::try_from(chunk).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        CameraUniform { cam_mat, clip_space }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CameraMatrix {
    pub cam_matrix: Matrix4<f32>,
    pub clip_space: Matrix4<f32>,
}

impl CameraMatrix {
    pub fn new(aspect_ratio: f32) -> Self {
        // view matrix (camera)
        let eye: nalgebra::OPoint<f32, nalgebra::Const<3>> = nalgebra::Point3::new(0.0, 0.0, 5.0); // Camera is 5 units back
        let target = nalgebra::Point3::new(0.0, 0.0, 0.0); // Looking at origin
        let up = nalgebra::Vector3::y(); // Up is +Y
        let cam_matrix = Matrix4::look_at_rh(&eye, &target, &up);
        
        // Create projection matrix
        let projection = geometry::Perspective3::new(
            aspect_ratio, 
            std::f32::consts::FRAC_PI_4, 
            0.1, 
            100.0
        ).to_homogeneous();
        
        // Combine them: projection * view
        let view_proj = projection * cam_matrix;
        Self {
            cam_matrix,
            clip_space: view_proj,
        }
    }

}

pub struct VertexShaders {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub render_pipeline: wgpu::RenderPipeline,
    pub cam_bind_group: wgpu::BindGroup,
}

impl VertexShaders {
    pub fn new(device: &wgpu::Device, config: wgpu::SurfaceConfiguration) -> Result<Self> {
        // config shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bind_group"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                // this is an array of the @group(0) attribute and then in that group is the @binding()
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            // THIS IS WHERE THE SETUP OF SHADERS IS BEING DONE.
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),

            // The primitive field describes how to interpret our vertices when converting them into triangles.
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, //Some(wgpu::Face::Back), // This will hide back faces
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            // Some(wgpu::DepthStencilState {
            //     format: wgpu::TextureFormat::Depth32Float,
            //     depth_write_enabled: true,
            //     depth_compare: wgpu::CompareFunction::Less,
            //     stencil: wgpu::StencilState::default(),
            //     bias: wgpu::DepthBiasState::default(),
            //  }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        //Camera Buffer

        let asp_ratio = config.width / config.height;
        let cam = CameraMatrix::new(asp_ratio as f32);
        println!("Camera Matrix: {:?}", cam.cam_matrix);
        println!("Camera Matrix to Clip Space: {:?}", cam.clip_space);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::bytes_of(&CameraUniform::from(cam)),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        //Bind group for Camera
        let cam_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Cam Bind Group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &camera_buffer,
                    offset: 0,
                    // this is not the size of th ebuffer but how much of the buffer you are binding to the shader.
                    size: Some(NonZeroU64::new(128).expect("buffer size must be non-zero")),
                }),
            }],
        });

        let mut cube_1 = Cube::new();
        let cube_2 = cube_1.to_world_space();
        println!("Cube 1: {:?}", cube_1.vertices);
        println!("Cube 2: {:?}", cube_2.vertices);


        let mut vertex_bytes = bytemuck::cast_slice(&cube_1.vertices).to_vec();
        vertex_bytes.extend_from_slice(bytemuck::cast_slice(&cube_2.vertices));

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: &vertex_bytes,
            usage: wgpu::BufferUsages::VERTEX,
            
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Index Buffer"),
            contents: bytemuck::cast_slice(&INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_vertices = (cube_1.vertices.len()+ cube_2.vertices.len()) as u32;
        print!("Number of vertices: {}\n", num_vertices);

        let num_indices = INDICES.len() as u32;

        Ok(Self {
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
            render_pipeline,
            cam_bind_group,
        })
    }
}
