use gl_matrix::mat4;

pub struct Camera {
    pub projection_matrix: [f32; 16],
    aspect_ratio: f32,
    clip_near: f32,
    clip_far: f32,
    fov: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, clip_near: f32, clip_far: f32, fov: f32) -> Camera {
        Camera {
            projection_matrix: generate_projection_matrix(aspect_ratio, clip_near, clip_far, fov),
            aspect_ratio,
            clip_near,
            clip_far,
            fov
        }
    }

    pub fn update_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.projection_matrix = generate_projection_matrix(aspect_ratio, self.clip_near, self.clip_far, self.fov);
    }
}

fn generate_projection_matrix(aspect_ratio: f32, clip_near: f32, clip_far: f32, fov: f32) -> [f32; 16] {
    let mut matrix = mat4::create();
    let fov_in_radians = fov * std::f32::consts::PI / 180.0;

    mat4::perspective(
        &mut matrix,
        fov_in_radians,
        aspect_ratio,
        clip_near,
        Some(clip_far),
    )
}