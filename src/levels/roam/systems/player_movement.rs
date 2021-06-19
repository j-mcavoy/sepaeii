use super::components::map_layers::*;
use super::components::*;
use crate::common::components::*;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_tiled_prototype::Map;
use tiled::LayerTile;

const VELOCITY: f32 = 300.;

const UP_OFFSETS: [(f32, f32, f32); 2] = [(-8., 0., 0.), (8., 0., 0.)];
const DOWN_OFFSETS: [(f32, f32, f32); 2] = [(-8., -8., 0.), (8., -8., 0.)];
const LEFT_OFFSETS: [(f32, f32, f32); 2] = [(-8., 0., 0.), (-8., -8., 0.)];
const RIGHT_OFFSETS: [(f32, f32, f32); 2] = [(8., 0., 0.), (8., -8., 0.)];
pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut set: QuerySet<(
        Query<(
            &PandaMan,
            &mut PandaManSprux,
            &mut Transform,
            &mut AnimationTimer,
        )>,
        Query<(&Camera, &mut Transform)>,
        Query<(&NPC, &Sprite)>,
    )>,
    maps_handle: Res<Assets<Map>>,
) {
    let delta = VELOCITY * time.delta_seconds();
    let mut next = Vec3::ZERO;

    for (_pandaman, mut walkable, mut sprite_transform, _animation_timer) in set.q0_mut().iter_mut()
    {
        for (_, map) in maps_handle.iter() {
            let objects_layer = map.map.layers.get(OBJECTS).unwrap();
            let object_layer = match objects_layer.tiles.clone() {
                tiled::LayerData::Finite(t) => t,
                _ => panic!("invalid map layer"),
            };
            let ground2_layer = map.map.layers.get(GROUND2).unwrap();
            let ground2_layer = match ground2_layer.tiles.clone() {
                tiled::LayerData::Finite(t) => t,
                _ => panic!("invalid map layer"),
            };
            let _direction = Vec3::ZERO;
            let curr_walkablestate = walkable.state;
            let next_walkablestate = if keyboard_input.pressed(KeyCode::W) {
                let temp_next = delta * Vec3::Y;
                if are_spaces_valid(
                    UP_OFFSETS
                        .iter()
                        .map(|offset| {
                            sprite_transform.translation + temp_next + offset.clone().into()
                        })
                        .collect::<Vec<Vec3>>(),
                    &object_layer,
                    &ground2_layer,
                ) {
                    next = temp_next;
                }
                WalkableState::WalkUp
            } else if keyboard_input.pressed(KeyCode::A) {
                let temp_next = delta * -Vec3::X;
                if are_spaces_valid(
                    LEFT_OFFSETS
                        .iter()
                        .map(|offset| {
                            sprite_transform.translation + temp_next + offset.clone().into()
                        })
                        .collect::<Vec<Vec3>>(),
                    &object_layer,
                    &ground2_layer,
                ) {
                    next = temp_next;
                }
                WalkableState::WalkLeft
            } else if keyboard_input.pressed(KeyCode::S) {
                let temp_next = delta * -Vec3::Y;
                if are_spaces_valid(
                    DOWN_OFFSETS
                        .iter()
                        .map(|offset| {
                            sprite_transform.translation + temp_next + offset.clone().into()
                        })
                        .collect::<Vec<Vec3>>(),
                    &object_layer,
                    &ground2_layer,
                ) {
                    next = temp_next;
                }
                WalkableState::WalkDown
            } else if keyboard_input.pressed(KeyCode::D) {
                let temp_next = delta * Vec3::X;
                if are_spaces_valid(
                    RIGHT_OFFSETS
                        .iter()
                        .map(|offset| {
                            sprite_transform.translation + temp_next + offset.clone().into()
                        })
                        .collect::<Vec<Vec3>>(),
                    &object_layer,
                    &ground2_layer,
                ) {
                    next = temp_next;
                }
                WalkableState::WalkRight
            } else {
                match curr_walkablestate {
                    WalkableState::WalkUp => WalkableState::StillUp,
                    WalkableState::WalkDown => WalkableState::StillDown,
                    WalkableState::WalkLeft => WalkableState::StillLeft,
                    WalkableState::WalkRight => WalkableState::StillRight,
                    _ => curr_walkablestate,
                }
            };
            sprite_transform.translation += next;

            if curr_walkablestate != next_walkablestate {
                walkable.reset_animation_strip();
            }
            walkable.state = next_walkablestate;
        }
    }
    for (_, mut transform) in set.q1_mut().iter_mut() {
        transform.translation += next;
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
