use macroquad::prelude::*;
use crate::config;

pub struct CameraController {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub target: Vec3,
    pub target_destination: Vec3,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            yaw: config::DEFAULT_CAMERA_YAW,
            pitch: config::DEFAULT_CAMERA_PITCH,
            distance: config::DEFAULT_CAMERA_DISTANCE,
            target: Vec3::ZERO,
            target_destination: Vec3::ZERO,
        }
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw += delta_x * config::CAMERA_ROTATION_SPEED;
        self.pitch = (self.pitch - delta_y * config::CAMERA_ROTATION_SPEED)
            .clamp(config::MIN_CAMERA_PITCH, config::MAX_CAMERA_PITCH);
    }

    pub fn zoom(&mut self, delta: f32) {
        self.distance = (self.distance - delta * config::CAMERA_ZOOM_SPEED)
            .clamp(config::MIN_CAMERA_DISTANCE, config::MAX_CAMERA_DISTANCE);
    }

    pub fn set_target(&mut self, target: Vec3) {
        self.target_destination = target;
    }

    pub fn update(&mut self) {
        self.target = self.target.lerp(self.target_destination, config::CAMERA_LERP_FACTOR);
    }

    pub fn reset_target(&mut self) {
        self.target_destination = Vec3::ZERO;
    }

    pub fn calculate_position(&self) -> Vec3 {
        Vec3::new(
            self.target.x + self.distance * self.yaw.cos() * self.pitch.cos(),
            self.target.y + self.distance * self.pitch.sin(),
            self.target.z + self.distance * self.yaw.sin() * self.pitch.cos(),
        )
    }

    pub fn to_camera3d(&self) -> Camera3D {
        Camera3D {
            position: self.calculate_position(),
            target: self.target,
            up: Vec3::Y,
            fovy: 45.0,
            projection: Projection::Perspective,
            ..Default::default()
        }
    }
}

impl Default for CameraController {
    fn default() -> Self {
        Self::new()
    }
}