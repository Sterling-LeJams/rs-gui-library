use anyhow::Result;
use crate::geometry::geometry::{Vertex, Cube, INDICES};
use crate::camera::camera::CameraMatrix;
use crate::camera::camera::CameraUniform;
use crate::shaders::buffers::BufferTypes;
use nalgebra::Point3;
use crate::shaders::bind_group::BindGrouping;

pub struct VertexShaders {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub render_pipeline: wgpu::RenderPipeline,
    pub camera_bytes: Option<wgpu::Buffer>,
    pub bind_group: wgpu::BindGroup,
}

impl VertexShaders {
    pub fn new(device: &wgpu::Device, config: wgpu::SurfaceConfiguration) -> Result<Self> {
        // config shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });

        let cube_1 = Cube::new();
        let cube_tran = cube_1.move_cube(Point3::new(-0.5, 0.8, 0.0));

        //Camera Buffer
        let asp_ratio = config.width / config.height;
        let cam = CameraMatrix::new(asp_ratio as f32);

        let cam_uniform = &CameraUniform::from(cam);
        let cam_bytes = bytemuck::bytes_of(cam_uniform);
        let camera_buffer = BufferTypes::UniformBuffer(cam_bytes).build(Some("camera"), &device);

        // TRANSLATION BUFFER
        let translation_mat_bytes = bytemuck::bytes_of(&cube_tran);
        let translation_buffer = BufferTypes::UniformBuffer(translation_mat_bytes).build(Some("translation buffer"), &device);

        let v_buff = vec![camera_buffer, translation_buffer];
        let bg = BindGrouping::new(device, &v_buff[..]);

        // let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //     label: Some("bind_group"),
        //     // CAMERA
        //     entries: &[wgpu::BindGroupLayoutEntry {
        //         binding: 0,
        //         visibility: wgpu::ShaderStages::VERTEX,
        //         ty: wgpu::BindingType::Buffer {
        //             ty: wgpu::BufferBindingType::Uniform,
        //             has_dynamic_offset: false,
        //             min_binding_size: None,
        //         },
        //         count: None,
        //     },
        //     // CUBE TRANSLATION 
        //    wgpu::BindGroupLayoutEntry {
        //         binding: 1,
        //         visibility: wgpu::ShaderStages::VERTEX,
        //         ty: wgpu::BindingType::Buffer {
        //             ty: wgpu::BufferBindingType::Uniform,
        //             has_dynamic_offset: false,
        //             min_binding_size: None,
        //         },
        //         count: None,
        //     }, 
        //     ],
        // });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                // this is an array of the @group(0) attribute and then in that group is the @binding()
                bind_group_layouts: &[&bg.bind_group.0],
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

        // //Bind group for Camera and Translation
        // let bind_group = device.create_bind_group(&BindGroupDescriptor {
        //     label: Some("Cam Bind Group"),
        //     layout: &bg.bind_group.0,
        //     // CAMERA
        //     entries: &[BindGroupEntry {
        //         binding: 0,
        //         resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
        //             buffer: &self.camera_buffer,
        //             offset: 0,
        //             // this is not the size of th ebuffer but how much of the buffer you are binding to the shader.
        //             // I AM USING None TO USE THE ENTIRE SIZE OF THE BUFFER PER THE DOCS
        //             size: None,
        //         }),
        //     }, 
        //     // TRANSLATION
        //     BindGroupEntry {
        //         binding: 1,
        //         resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
        //             buffer: &translation_buffer,
        //             offset: 0,
        //             // this is not the size of th ebuffer but how much of the buffer you are binding to the shader.
        //             size: None,
        //         }),
        //     }
        // ],
        // });

        let vertex_bytes:Vec<u8> = bytemuck::cast_slice(&cube_1.vertices).to_vec();

        let vertex_buffer = BufferTypes::VertexBuffer(&vertex_bytes).build(Some("v1"), &device);

        let index_buffer = BufferTypes::IndexBuffer(&INDICES).build(Some("indices"), &device);

        let num_vertices = cube_1.vertices.len() as u32;

        let num_indices = INDICES.len() as u32;

        Ok(Self {
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
            render_pipeline,
            camera_bytes: None,
            bind_group: bg.bind_group.1,
        })
    }

}





