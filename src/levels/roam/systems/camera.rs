use super::components::*;
use bevy::prelude::*;
use bevy::render::camera::Camera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_xyz(SPAWN_POINT.0, SPAWN_POINT.1, 100.),
        ..OrthographicCameraBundle::new_2d()
    });
}

pub fn camera_movement(
    mut set: QuerySet<(
        Query<&Transform, With<PandaMan>>,
        Query<&mut Transform, With<Camera>>,
        Query<&mut Transform, With<MainMenu>>,
    )>,
) {
    let mut transform = Transform::default();
    for player_transform in set.q0().single() {
        transform = player_transform.clone();
    }
    for mut camera_transform in set.q1_mut().single_mut() {
        camera_transform.translation.x = transform.translation.x;
        camera_transform.translation.y = transform.translation.y;
    }
}
