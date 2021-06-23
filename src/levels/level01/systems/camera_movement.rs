use super::components::*;
use bevy::prelude::*;
use bevy::render::camera::Camera;

pub fn camera_movement(
    mut set: QuerySet<(
        Query<(&Mario, &Transform)>,
        Query<(&Camera, &mut Transform)>,
    )>,
) {
    let mut panda_translation = Vec3::ZERO;
    for (_mario, transform) in set.q0().iter() {
        panda_translation = transform.translation;
    }
    for (camera, mut camera_transform) in set.q1_mut().iter_mut() {
        camera_transform.translation = camera_transform
            .translation
            .lerp(panda_translation + Vec3::new(25., 50., 0.), 0.05);
    }
}
