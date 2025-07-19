use anyhow::Result;
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindGroupLayout};
use nalgebra::geometry;

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
                }
            ]
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    //Front face (z = 0.5)
    Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 1.0] }, // 4
    Vertex { position: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 1.0] }, // 5
    Vertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 1.0, 1.0] }, // 6
    Vertex { position: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 0.0] }, // 7

        // Back face (z = -0.5)
    Vertex { position: [-1.5, -1.5, -0.5], color: [1.0, 0.0, 0.0] }, // 4
    Vertex { position: [ 1.5, -1.5, -0.5], color: [0.0, 1.0, 0.0] }, // 5
    Vertex { position: [ 1.5,  1.5, -0.5], color: [0.0, 0.0, 1.0] }, // 6
    Vertex { position: [-1.5,  1.5, -0.5], color: [1.0, 1.0, 0.0] }, // 7
    ];

// to draw triangles on each square face
pub const INDICES: &[u16] = &[
    
    // Front face
    0, 1, 2,
    2, 3, 0,

    //Back Face
    5, 4, 7,
    7, 6, 5, 
];

// should probably have some sort of erro handeling ot match the vertices with the indices

// need to have a const to have a projection matrices to convert the model/local space into clip space
// supposed to be a library here
//pub const PROJECTION_MATR: 

pub struct CameraMatrix {
    pub proj_mat: geometry::Perspective3<f32>

}

pub const CAMERA_MAT:CameraMatrix = CameraMatrix { proj_mat: geometry::Perspective3::new(
    aspect: 16.0/9.0,
    fovy: std::f32::consts::FRAC_PI_3,
    znear 0.2,
    zfar 100.0,  
    )
}; 

pub struct VertexShaders {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub render_pipeline: wgpu::RenderPipeline,
    pub bind_buffer_group: wgpu::BindGroup,
    
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
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None, 
                }
            ],
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            // this is an array of the @group(0) attribute and then in that group is the @binding() 
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            // THIS IS WHERE THE PROCESSING OF THE SHADERS IS DONE
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), 
                buffers: &[Vertex::desc(),], 
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { // 4.
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
                cull_mode: None,//Some(wgpu::Face::Back), // This will hide back faces
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            }, depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
             }),
            multisample: wgpu::MultisampleState {
                count: 1, 
                mask: !0, 
                alpha_to_coverage_enabled: false, 
            },
            multiview: None, 
            cache: None, 
            });

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Cube Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let bind_buffer_group = device.create_buffer_init(
            &wgpu::util:BufferInitDescriptor {
            label: Some("Binding Buffer"),
            contents ,
            usage: wgpu::BufferUsages::UNIFORM,
        })
        //NEED TO CREATE THE ACTUAL CAMERA MATRIX AND THEN CONTENTS IS THE NUMBER OF BYTES

        let num_vertices = VERTICES.len() as u32;
        print!( "Number of vertices: {}\n", num_vertices);

        let num_indices = INDICES.len() as u32;
        Ok(Self {
            vertex_buffer,
            index_buffer,  
            num_vertices,
            num_indices,
            render_pipeline,
            bind_buffer_group
        })
    }
}

