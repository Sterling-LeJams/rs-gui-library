use wgpu::Color;
// 2D Color Texture	Regular images or UI textures
// Depth Texture	Used for depth testing during rendering
// Cube Map	Skyboxes, reflections
// 3D Volume Texture	Medical data, procedural volume effects
// 2D Texture Array	Sprite atlases, cascaded shadow maps
// Storage Texture	Read/write access in compute shaders
// Multisampled Texture	For anti-aliased rendering
// Stencil/Depth-Stencil
use wgpu::TextureDescriptor;
use wgpu::TextureView;
use std::sync::Arc;

pub enum TextureType {
    Color(ColorTexture),
    Depth(DepthTexture),
    Volume(VolumeTexture),
    TextureArray(TextureArray),
    Storage(StorageTexture),
    Multisampled(MultisampledTexture),
    StencilDepth(StencilDepthTexture),

}

// pub trait TextureProperties {
//     fn get_properties(&self) -> Option<(wgpu::TextureView, wgpu::TextureFormat)>;
// }

// impl TextureProperties for ColorTexture {
//     fn get_properties(&self) -> Option<(wgpu::TextureView, wgpu::TextureFormat)> {
//         // Placeholder implementation
//         None
//     }
// }
// impl TextureProperties for DepthTexture {
//     fn get_properties(&self) -> Option<(wgpu::TextureView, wgpu::TextureFormat)> {
//         // Placeholder implementation
//         None
//     }
// }
// impl TextureProperties for CubeMapTexture {
//     fn get_properties(&self) -> Option<(wgpu::TextureView, wgpu::TextureFormat)> {
//         // Placeholder implementation
//         None
//     }
// }
// impl TextureProperties for VolumeTexture {
//     fn get_properties(&self) -> Option<(wgpu::TextureView, wgpu::TextureFormat)> {
//         // Placeholder implementation
//         None
//     }
// }
// impl TextureProperties for TextureArray {
//     fn get_properties(&self) -> Option<(wgpu::TextureView, wgpu::TextureFormat)> {
//         // Placeholder implementation
//         None
//     }
// }
// impl TextureProperties for StorageTexture {
//     fn get_properties(&self) -> Option<(wgpu::TextureView, wgpu::TextureFormat)> {
//         // Placeholder implementation
//         None                                    
//     }
// }
// impl TextureProperties for MultisampledTexture {
//     fn get_properties(&self) -> Option<(wgpu::TextureView, wgpu::TextureFormat)> {
//         // Placeholder implementation
//         None
//     }
// }   
// impl TextureProperties for StencilDepthTexture {
//     fn get_properties(&self) -> Option<(wgpu::TextureView, wgpu::TextureFormat)> {
//         None
//     }
// }   


pub trait TextureProperties {
    fn view(&self) -> Arc<wgpu::TextureView>;
    fn format(&self) -> wgpu::TextureFormat;
}

impl TextureProperties for ColorTexture {
    fn view(&self) -> Arc<wgpu::TextureView> {
        self.view.clone()
    }
 
    fn format(&self) -> wgpu::TextureFormat {
        self.format
    }
}

impl TextureProperties for DepthTexture {
    fn view(&self) -> Arc<wgpu::TextureView> {
        self.view.clone()
    }

    fn format(&self) -> wgpu::TextureFormat {
        self.format
    }
}
impl TextureProperties for VolumeTexture {
    fn view(&self) -> Arc<wgpu::TextureView> {
        self.view.clone()
    }

    fn format(&self) -> wgpu::TextureFormat {
        self.format
    }
}
impl TextureProperties for TextureArray {
    fn view(&self) -> Arc<wgpu::TextureView> {
        self.view.clone()
    }

    fn format(&self) -> wgpu::TextureFormat {
        self.format
    }
}
impl TextureProperties for StorageTexture {
    fn view(&self) -> Arc<wgpu::TextureView> {
        self.view.clone()
    }
    fn format(&self) -> wgpu::TextureFormat {
        self.format
    }
}
impl TextureProperties for MultisampledTexture {
    fn view(&self) -> Arc<wgpu::TextureView> {
        self.view.clone()
    }

    fn format(&self) -> wgpu::TextureFormat {
        self.format
    }
}   
impl TextureProperties for StencilDepthTexture {
    fn view(&self) -> Arc<wgpu::TextureView> {
        self.view.clone()
    }

    fn format(&self) -> wgpu::TextureFormat {
        self.format
    }
}   

impl TextureType {
    pub fn info(&self) -> &dyn TextureProperties {
        match self {
            TextureType::Color(t)  => t,
            TextureType::Depth(t)  => t,
            TextureType::Volume(t) => t,
            TextureType::TextureArray(t) => t,
            TextureType::Storage(t) => t,
            TextureType::Multisampled(t) => t,
            TextureType::StencilDepth(t) => t,
        }
    }
}

pub struct ColorTexture {
    pub view: Arc<wgpu::TextureView>,
    pub format: wgpu::TextureFormat,
}
pub struct DepthTexture {
     pub view: Arc<wgpu::TextureView>,
    pub format: wgpu::TextureFormat,
}
pub struct CubeMapTexture {
    pub view: Arc<wgpu::TextureView>,
    pub format: wgpu::TextureFormat,
}
pub struct VolumeTexture {
     pub view: Arc<wgpu::TextureView>,
    pub format: wgpu::TextureFormat,
}
pub struct TextureArray {
     pub view: Arc<wgpu::TextureView>,
    pub format: wgpu::TextureFormat,
}
pub struct StorageTexture {
    pub view: Arc<wgpu::TextureView>,
    pub format: wgpu::TextureFormat,
}
pub struct MultisampledTexture  {
    pub view: Arc<wgpu::TextureView>,
    pub format: wgpu::TextureFormat,
}

pub struct StencilDepthTexture {
    pub view: Arc<wgpu::TextureView>,
    pub format: wgpu::TextureFormat,
}

impl StencilDepthTexture {
    pub fn new(config:&wgpu::SurfaceConfiguration, device: &wgpu::Device) -> Self {

        let frmt = wgpu::TextureFormat::Depth32Float;
        let depth_texture = device.create_texture(&TextureDescriptor {
        size: wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
            },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: frmt,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        label: Some("Depth Texture"),
        view_formats: &[],
        });

        let depth_view: wgpu::TextureView = depth_texture.create_view(&Default::default());

        Self {
            view: Arc::new(depth_view),
            format: frmt,
        }
 
    }
}