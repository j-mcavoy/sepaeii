use super::SPAWN_POINT;
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    let transform = Transform {
        scale: Vec3::splat(1. / 2.),
        translation: Vec3::new(SPAWN_POINT.0, SPAWN_POINT.1, 100.),
        ..Default::default()
    };
    commands.spawn_bundle(OrthographicCameraBundle {
        transform,
        ..OrthographicCameraBundle::new_2d()
    });
}
