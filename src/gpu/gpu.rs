use wgpu::Instance;
use anyhow::Result;
use wgpu::TextureDescriptor;

pub struct GPUDevice {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
}

impl GPUDevice {
    pub async fn new<'window>(surface: &wgpu::Surface<'window>, instance: Instance) -> Result<Self> {
        
        // Adapters can be used to open a connection to the corresponding Device
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        // Open connection to a graphics and/or compute device.
        // Responsible for the creation of most rendering and compute resources.
        // These are then used in commands, which are submitted to a [`Queue`].    
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

            Ok(Self {
                instance,
                device,
                queue,
                adapter,
            })
    }

    pub fn create_depth_stencil_texture_and_view(&self, config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView { 
        let depth_texture = self.device.create_texture(&TextureDescriptor {
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("Depth Texture"),
            view_formats: &[],
    });
 
        let depth_view = depth_texture.create_view(&Default::default());

        depth_view

    }
}
