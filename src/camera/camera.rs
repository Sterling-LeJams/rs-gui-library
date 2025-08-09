use nalgebra::Matrix4;
use nalgebra::geometry::Perspective3;

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
        let projection = Perspective3::new(
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


