use super::components::*;
use bevy::prelude::*;
use bevy::render::camera::Camera;

const VELOCITY: f32 = 0.1;
pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut set: QuerySet<(
        Query<(&PandaMan, &mut Walkable, &mut Transform)>,
        Query<(&Camera, &mut Transform)>,
    )>,
) {
    let mut direction = Vec3::ZERO;
    for (_pandaman, mut walkable, mut sprite_transform) in set.q0_mut().iter_mut() {
        if keyboard_input.pressed(KeyCode::X) {
            direction -= Vec3::new(0.0, 0.0, 1.0);
        } else if keyboard_input.pressed(KeyCode::Z) {
            direction += Vec3::new(0.0, 0.0, 1.0);
        } else if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
            walkable.state = WalkableState::WalkLeft;
        } else if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            walkable.state = WalkableState::WalkRight;
        } else if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            walkable.state = WalkableState::WalkUp;
        } else if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
            walkable.state = WalkableState::WalkDown;
        } else {
            walkable.state = match walkable.state {
                WalkableState::WalkUp => WalkableState::StillUp,
                WalkableState::WalkDown => WalkableState::StillDown,
                WalkableState::WalkLeft => WalkableState::StillLeft,
                WalkableState::WalkRight => WalkableState::StillRight,
                _ => walkable.state.clone(),
            };
        }

        sprite_transform.translation += time.delta_seconds() * direction * VELOCITY * 1000.;
        println!("{:?}", sprite_transform);
    }
    for (_, mut transform) in set.q1_mut().iter_mut() {
        println!("direction map {:?}", direction);
        transform.translation += time.delta_seconds() * direction * VELOCITY * 1000.;
        println!("{:?}", transform);
    }
}
