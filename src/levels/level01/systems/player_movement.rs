use std::f32::INFINITY;

use super::super::setup::SPAWN_POINT;
use super::components::map_layers::*;
use super::components::*;
use crate::common::components::*;
use crate::levels::level01::setup::{TILE_HEIGHT, TILE_WIDTH};
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_tiled_prototype::Map;
use tiled::LayerTile;

use MarioState::*;
use PowerUp::*;

const JUMP_VELOCITY: f32 = 200.;
const WALK_ACCEL: f32 = 900.;
const WALK_DECELL_FACTOR: f32 = 0.09;
const WALK_MAX_VELOCITY: f32 = 200.;
const GRAVITY: f32 = 400.;

const UP_OFFSETS: [(f32, f32, f32); 2] = [(-8., 0., 0.), (8., 0., 0.)];
const DOWN_OFFSETS: [(f32, f32, f32); 2] = [(-8., -8., 0.), (8., -8., 0.)];
const LEFT_OFFSETS: [(f32, f32, f32); 2] = [(-8., 0., 0.), (-8., -8., 0.)];
const RIGHT_OFFSETS: [(f32, f32, f32); 2] = [(8., 0., 0.), (8., -8., 0.)];

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut Mario,
        &mut MarioSpriteplex,
        &mut Transform,
        &mut AnimationTimer,
    )>,
    maps_handle: Res<Assets<Map>>,
) {
    for (mut mario, mut spriteplex, mut transform, animation_timer) in query.iter_mut() {
        for (_, map) in maps_handle.iter() {
            let delta = time.delta_seconds();

            mario.velocity.x += if keyboard_input.pressed(KeyCode::A) {
                -WALK_ACCEL * delta
            } else if keyboard_input.pressed(KeyCode::D) {
                WALK_ACCEL * delta
            } else {
                0.
            };
            mario.velocity.x += -mario.velocity.x * WALK_DECELL_FACTOR;

            let objects_layer = map.map.layers.get(OBJECTS).unwrap();
            let objects_layer = match objects_layer.tiles.clone() {
                tiled::LayerData::Finite(t) => t,
                _ => panic!("invalid map layer"),
            };

            println!("Grounded: {:?}", mario.is_grounded);
            println!("Velocity: {:?}", mario.velocity);
            println!("Translation: {:?}", transform.translation);

            if mario.is_grounded {
                println!("On Ground");
                mario.velocity.y = 0.;
            }
            if keyboard_input.just_pressed(KeyCode::Return) && mario.is_grounded {
                println!("Jumped");
                mario.velocity.y = JUMP_VELOCITY;
                mario.is_grounded = false;
            } else {
                mario.velocity.y += -GRAVITY * delta;
            }
            if mario.velocity.y < -0. && !mario.is_grounded {
                mario.is_grounded = check_if_on_ground(vec![transform.translation], &objects_layer);
                if mario.is_grounded {
                    println!("Just Grounded");
                }
            }

            mario.velocity = mario.velocity.clamp(
                Vec3::new(-WALK_MAX_VELOCITY, -JUMP_VELOCITY, 0.0),
                Vec3::new(WALK_MAX_VELOCITY, JUMP_VELOCITY, 0.0),
            );
            transform.translation +=
                Vec3::new(mario.velocity.x * delta, mario.velocity.y * delta, 0.);
            transform.translation = transform
                .translation
                .clamp(Vec3::new(0., -26., 0.), Vec3::splat(100. * 32.));

            spriteplex.state = mario.clone().next_state();
        }
    }
}

fn check_if_on_ground(player_coords: Vec<Vec3>, object_layer: &Vec<Vec<LayerTile>>) -> bool {
    println!("{:?}", object_layer[25][25]);
    println!("player_coords: {:?}", player_coords);
    let out = player_coords
        .iter()
        .map(|vec3| coord2map_index(&(vec3.x, vec3.y).into()))
        .inspect(|elm| println!("elm {:?}", elm))
        .inspect(|elm| println!("gid {:?}", object_layer[elm.1][elm.0].gid))
        .all(|(x, y)| object_layer[y][x].gid != 0);
    out
}

pub fn coord2map_index(coord: &Vec2) -> (usize, usize) {
    (
        (coord.x / TILE_WIDTH).clamp(0., 1100.) as usize,
        (-coord.y).clamp(0., 28.) as usize,
    )
}
