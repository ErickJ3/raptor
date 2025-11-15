use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn from_camera(screen_pos: Vec2, camera: &Camera3D) -> Self {
        let ndc_x = (2.0 * screen_pos.x) / screen_width() - 1.0;
        let ndc_y = 1.0 - (2.0 * screen_pos.y) / screen_height();

        let fovy_rad = camera.fovy.to_radians();
        let aspect = screen_width() / screen_height();

        let forward = (camera.target - camera.position).normalize();
        let right = forward.cross(camera.up).normalize();
        let up = right.cross(forward).normalize();

        let half_height = (fovy_rad / 2.0).tan();
        let half_width = half_height * aspect;

        let ray_dir = (forward + right * ndc_x * half_width + up * ndc_y * half_height).normalize();

        Self {
            origin: camera.position,
            direction: ray_dir,
        }
    }
}
