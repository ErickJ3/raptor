use super::ray::Ray;
use macroquad::prelude::*;

/// AABB
pub fn ray_box_intersection(ray: &Ray, center: Vec3, half_size: Vec3) -> Option<f32> {
    let min = center - half_size;
    let max = center + half_size;

    let mut tmin = (min.x - ray.origin.x) / ray.direction.x;
    let mut tmax = (max.x - ray.origin.x) / ray.direction.x;

    if tmin > tmax {
        std::mem::swap(&mut tmin, &mut tmax);
    }

    let mut tymin = (min.y - ray.origin.y) / ray.direction.y;
    let mut tymax = (max.y - ray.origin.y) / ray.direction.y;

    if tymin > tymax {
        std::mem::swap(&mut tymin, &mut tymax);
    }

    if tmin > tymax || tymin > tmax {
        return None;
    }

    tmin = tmin.max(tymin);
    tmax = tmax.min(tymax);

    let mut tzmin = (min.z - ray.origin.z) / ray.direction.z;
    let mut tzmax = (max.z - ray.origin.z) / ray.direction.z;

    if tzmin > tzmax {
        std::mem::swap(&mut tzmin, &mut tzmax);
    }

    if tmin > tzmax || tzmin > tmax {
        return None;
    }

    tmin = tmin.max(tzmin);

    if tmin > 0.0 { Some(tmin) } else { None }
}

/// 3d -> 2D
pub fn world_to_screen(world_pos: Vec3, camera: &Camera3D) -> Option<Vec2> {
    let view = Mat4::look_at_rh(camera.position, camera.target, camera.up);
    let proj = Mat4::perspective_rh_gl(
        camera.fovy.to_radians(),
        screen_width() / screen_height(),
        0.1,
        1000.0,
    );

    let clip = proj * view * world_pos.extend(1.0);

    if clip.w <= 0.0 {
        return None;
    }

    let ndc = clip.truncate() / clip.w;

    if ndc.z < -1.0 || ndc.z > 1.0 {
        return None;
    }

    let screen_x = (ndc.x + 1.0) * 0.5 * screen_width();
    let screen_y = (1.0 - ndc.y) * 0.5 * screen_height();

    Some(Vec2::new(screen_x, screen_y))
}
