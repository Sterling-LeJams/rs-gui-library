use wgpu::{BindGroup, BindGroupLayout, BindGroupEntry, BindGroupLayoutEntry, Buffer, BindGroupDescriptor};

pub struct BindGrouping {
    pub bind_group: (BindGroupLayout, BindGroup)

}

impl BindGrouping {
   pub fn new(device: &wgpu::Device, buffer: &[Buffer]) -> Self{
        let bg= BindingGroupSetup::new(buffer);

            let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("bind_group"),
                entries: &bg.layout[..]
            });

            let bind_group = device.create_bind_group(&BindGroupDescriptor {
                label: Some("Cam Bind Group"),
                layout: &bind_group_layout,
                entries: &bg.bg_entry[..]
        });
        Self {
            bind_group: (bind_group_layout, bind_group)
        }
    } 
}

struct BindingGroupSetup<'a> {
    layout: Vec<BindGroupLayoutEntry>,
    bg_entry: Vec<BindGroupEntry<'a>>,
}

impl <'a> BindingGroupSetup<'a> {
    fn new(buffer: &'a [Buffer]) -> Self {
        let layout: Vec<_> = buffer.into_iter().enumerate().map(|(i, _b)|{
            BindGroupLayoutEntry {
            binding: i as u32,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
        },
        count: None, 
            }
        }).collect();

       let bg_entry: Vec<_> = buffer.into_iter().enumerate().map(|(i, b)|{
            BindGroupEntry {
                binding: i as u32,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: b,
                    offset: 0,
                    size: None,
                    }),
                }
        }).collect();

        Self {
            layout,
            bg_entry
        }
    }
}