use std::{sync::Arc};
use anyhow::Result;
use wgpu::Operations;
use crate::shaders::geometry::VertexShaders;
use crate::gpu::gpu::GPUDevice;
use wgpu::SurfaceConfiguration;
use wgpu::RenderPassDepthStencilAttachment;
use std::time;
use crate::textures::textures::*;
use wgpu::TextureView;


use winit::{
    event_loop::ActiveEventLoop,
    keyboard::KeyCode,
    window::Window,
};

pub struct WindowState {
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    pub window: Arc<Window>,
    gpu: GPUDevice,
    vertex_shaders: VertexShaders,

    //i do not like this but this is just a testing
    stencil_depth: Arc<TextureView>,

}

impl WindowState {
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        // The instance creates the backend for the GPU. Backends::PRIMARY; this includes Metal 
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let gpu = GPUDevice::new(&surface, instance).await?;

        let config = configure_surface(&gpu.adapter, &window, &surface);

        let vertex_shaders = VertexShaders::new(&gpu.device, config.clone())?;
        let stencil_depth = TextureType::StencilDepth(StencilDepthTexture::new(&config, &gpu.device)).info().view();

        Ok(Self {
            surface,
            config: config.clone(),
            is_surface_configured: false,
            window,
            gpu,
            vertex_shaders,
            stencil_depth,
        })
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        // remember this is a continous loop so everything in here is being looped
        self.window.request_redraw();
        println!("Rendering frame - Window ID: {:?} Time: {:?}", self.window.id(), time::SystemTime::now());
  
        // Remember render() is called every time the window is redrawn, so we need to check if the surface is configured before proceeding. so if 
        // it cant draw a frame it will just exit with an Ok(()) instead of throwing an error.
        if !self.is_surface_configured {
            return Ok(());
        }
        
        let output = self.surface.get_current_texture()?;
        
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // We also need to create a CommandEncoder to create the actual commands to send to the GPU. Most modern graphics frameworks expect commands to be stored in a command buffer before being sent to the GPU. 
        // The encoder builds a command buffer that we can then send to the GPU.
        // Command buffers are objects used to record commands which can be subsequently submitted to a device queue for execution. Hold GPU actions to send to the Device like Clear the screen, draw triangles, copy a texture etc.
    
        let mut encoder = self.gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),});

        // Now we can get to clearing the screen a long time coming. We need to use the encoder to create a RenderPass. The RenderPass has all the methods for the actual drawing. 
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &self.stencil_depth,
                    //messing with this after adding a depth stencil attachment finally go something to render
                    depth_ops:Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Discard,
                        }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.vertex_shaders.render_pipeline); // 2.
            render_pass.set_vertex_buffer(0, self.vertex_shaders.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.vertex_shaders.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            //render_pass.draw(0..self.vertex_shaders.num_vertices, 0..1); 
            render_pass.draw_indexed(0..self.vertex_shaders.num_indices, 0,0..1);

        drop(render_pass);

    // submit will accept anything that implements IntoIter
    self.gpu.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        // If we want to support resizing in our application, we're going to need to reconfigure 
        // the surface every timR the window's size changes. 
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.gpu.device, &self.config);
            self.is_surface_configured = true;
        } else {
            eprintln!("Surface is not configured yet, cannot resize.");
        }
    }

    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
    match (code, is_pressed) {
        (KeyCode::Escape, true) => event_loop.exit(),
        _ => {}
        
        }
    }
}

pub fn configure_surface(adapter: &wgpu::Adapter, window: &Window, surface: &wgpu::Surface<'_>) -> SurfaceConfiguration {
    let size = window.inner_size();
    let surface_caps = surface.get_capabilities(&adapter);
    
    let surface_format = surface_caps.formats.iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    config   
}











