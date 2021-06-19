use super::SPAWN_POINT;
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_xyz(SPAWN_POINT.0, SPAWN_POINT.1, 100.),
        ..OrthographicCameraBundle::new_2d()
    });
}
