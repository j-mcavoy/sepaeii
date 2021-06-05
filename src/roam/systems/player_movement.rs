use std::time::Duration;

use super::components::*;
use bevy::prelude::*;
use bevy::render::camera::Camera;

const VELOCITY: f32 = 200.;
pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut set: QuerySet<(
        Query<(
            &PandaMan,
            &mut Walkable,
            &mut Transform,
            &mut AnimationTimer,
        )>,
        Query<(&Camera, &mut Transform)>,
    )>,
) {
    let mut delta = Vec3::ZERO;

    for (_pandaman, mut walkable, mut sprite_transform, mut animation_timer) in
        set.q0_mut().iter_mut()
    {
        let mut direction = Vec3::ZERO;
        let curr_walkablestate = walkable.state;
        let next_walkablestate = if keyboard_input.pressed(KeyCode::A) {
            WalkableState::WalkLeft
        } else if keyboard_input.pressed(KeyCode::D) {
            WalkableState::WalkRight
        } else if keyboard_input.pressed(KeyCode::W) {
            WalkableState::WalkUp
        } else if keyboard_input.pressed(KeyCode::S) {
            WalkableState::WalkDown
        } else {
            match curr_walkablestate {
                WalkableState::WalkUp => WalkableState::StillUp,
                WalkableState::WalkDown => WalkableState::StillDown,
                WalkableState::WalkLeft => WalkableState::StillLeft,
                WalkableState::WalkRight => WalkableState::StillRight,
                _ => curr_walkablestate,
            }
        };

        if curr_walkablestate != next_walkablestate {
            walkable.reset_animation_strip();
        }

        delta = time.delta_seconds()
            * VELOCITY
            * match curr_walkablestate {
                WalkableState::WalkUp => Vec3::Y,
                WalkableState::WalkDown => -Vec3::Y,
                WalkableState::WalkRight => Vec3::X,
                WalkableState::WalkLeft => -Vec3::X,
                _ => Vec3::ZERO,
            };

        sprite_transform.translation += delta;
        walkable.state = next_walkablestate;
    }
    for (_, mut transform) in set.q1_mut().iter_mut() {
        transform.translation += delta;
    }
}
