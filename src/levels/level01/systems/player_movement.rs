use super::components::*;
use crate::common::components::*;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_tiled_prototype::Map;
use tiled::LayerTile;

use MarioState::*;
use PowerUp::*;

const JUMP_VELOCITY: f32 = 500.;
const WALK_ACCEL: f32 = 300.;
const WALK_MAX_VELOCITY: f32 = 300.;
const GRAVITY: f32 = 1000.;

const UP_OFFSETS: [(f32, f32, f32); 2] = [(-8., 0., 0.), (8., 0., 0.)];
const DOWN_OFFSETS: [(f32, f32, f32); 2] = [(-8., -8., 0.), (8., -8., 0.)];
const LEFT_OFFSETS: [(f32, f32, f32); 2] = [(-8., 0., 0.), (-8., -8., 0.)];
const RIGHT_OFFSETS: [(f32, f32, f32); 2] = [(8., 0., 0.), (8., -8., 0.)];

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut Mario,
        &MarioSpriteplex,
        &mut Transform,
        &mut AnimationTimer,
    )>,
    maps_handle: Res<Assets<Map>>,
) {
    for (mut mario, MarioSpriteplex { mut state, .. }, mut transform, animation_timer) in
        query.iter_mut()
    {
        println!("{:?}", transform.translation);
        let delta = time.delta_seconds();
        mario.velocity.y = if transform.translation.y > 0. {
            mario.velocity.y - GRAVITY * delta
        } else {
            0.
        };
        if keyboard_input.just_pressed(KeyCode::Return) && mario.is_grounded {
            mario.velocity.y = JUMP_VELOCITY;
            mario.is_grounded = false;
        }
        transform.translation += Vec3::new(mario.velocity.x * delta, mario.velocity.y * delta, 0.);
        transform.translation = transform
            .translation
            .clamp(Vec3::ZERO, Vec3::splat(100. * 32.));

        if transform.translation.y <= 0.0 {
            mario.is_grounded = true;
        }
    }
}
/*
        if keyboard_input.just_pressed(KeyCode::Space) && movements.eq(&Grounded) {
            movements = Jumping;
        }

        let state = match (keyboard_input.pressed(KeyCode::S), is_big, powerup) {
            (true, false, None) => SmallMarioCrouchRight,
            (true, false, Some(Star)) => SmallStarCrouch,
            (true, false, Some(FirePower)) => SmallFirePowerCrouchRight,
            (true, true, None) => BigMarioCrouchLeft,
            (true, true, Some(Star)) => BigStarCrouchRight,
            (true, true, Some(FirePower)) => BigFirePowerCrouchRight,
            (_, _, _) => match (state, velocity.x) {
                (SmallMarioCrouchRight, x) if x > 0.0 => SmallStarWalkRight,
                (SmallMarioCrouchRight, x) => SmallMarioCrouchLeft,
                (SmallMarioCrouchRight, 1.0) => SmallMarioCrouchRight,
            },
        };

        match (
            keyboard_input.pressed(KeyCode::A),
            keyboard_input.pressed(KeyCode::D),
            is_big,
            powerup,
            movements,
        ) {
            (true, _, _, _, _, _) => {}
            _ => (),
        };
    }
}

fn are_spaces_valid(
    player_coords: Vec<Vec3>,
    object_layer: &Vec<Vec<LayerTile>>,
    ground2_layer: &Vec<Vec<LayerTile>>,
) -> bool {
    print!("player_coords: {:?}", player_coords);
    let out = player_coords
        .iter()
        .inspect(|x| println!("{:?}", x))
        .map(|vec3| coord2map_index(&(vec3.x, vec3.y).into()))
        .inspect(|x| println!("{:?}", x))
        .all(|(x, y)| ground2_layer[y][x].gid != 0 || object_layer[y][x].gid == 0);
    if !out {
        println!("INVALID MOVE");
    }
    out
}

pub fn coord2map_index(coord: &Vec2) -> (usize, usize) {
    ((coord.x / 32.) as usize, (-coord.y / 32.) as usize)
}
        */
