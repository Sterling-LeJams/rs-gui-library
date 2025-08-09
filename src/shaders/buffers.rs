use wgpu::{util::{BufferInitDescriptor, DeviceExt}, Device};
use wgpu::Buffer;
use bytemuck::{Pod, cast_slice};

pub enum BufferTypes<'a, T: Pod> {
    VertexBuffer(&'a [T]),
    IndexBuffer(&'a [T]),
}

impl<'a, T: Pod> BufferTypes<'a, T> {
    pub fn build(&self, label: Option<&'a str>, device: &wgpu::Device) -> wgpu::Buffer {
        match self {
            BufferTypes::VertexBuffer(contents) => {
                device.create_buffer_init(&BufferInitDescriptor {
                    label,
                    contents: cast_slice(contents),
                    usage: wgpu::BufferUsages::VERTEX,
                })
            }
            BufferTypes::IndexBuffer(contents) => {
                device.create_buffer_init(&BufferInitDescriptor {
                    label,
                    contents: cast_slice(contents),
                    usage: wgpu::BufferUsages::INDEX,
                })
            }
        }
    }
}
