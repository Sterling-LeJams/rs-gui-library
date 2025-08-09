use anyhow::Result;
use std::{num::NonZeroU64};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry,
    util::{DeviceExt},
};
use crate::geometry::geometry::{Vertex, Cube, INDICES};
use crate::camera::camera::CameraMatrix;
use crate::camera::camera::CameraUniform;
use crate::shaders::buffers::BufferTypes;

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

        let mut vertex_bytes:Vec<u8> = bytemuck::cast_slice(&cube_1.vertices).to_vec();
        vertex_bytes.extend_from_slice(bytemuck::cast_slice(&cube_2.vertices));

        let vertex_buffer = BufferTypes::VertexBuffer(&vertex_bytes).build(Some("v1"), &device);

        let index_buffer = BufferTypes::IndexBuffer(&INDICES).build(Some("indices"), &device);

        let num_vertices = (cube_1.vertices.len()+ cube_2.vertices.len()) as u32;

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





