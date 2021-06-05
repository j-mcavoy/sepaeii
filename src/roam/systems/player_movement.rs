use super::components::map_layers::*;
use super::components::*;
use bevy::prelude::*;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_tiled_prototype::Map;
use bevy_tiled_prototype::MapReadyEvent;
use bevy_tiled_prototype::TiledMapBundle;
use tiled::LayerData;
use tiled::LayerTile;

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
    asset_server: Res<AssetServer>,
    maps_handle: Res<Assets<Map>>,
) {
    let mut delta = Vec3::ZERO;

    for (_pandaman, mut walkable, mut sprite_transform, _animation_timer) in set.q0_mut().iter_mut()
    {
        let _direction = Vec3::ZERO;
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

        for (_, map) in maps_handle.iter() {
            let layer = map.map.layers.get(OBJECTS).unwrap();
            let tile_vec = match layer.tiles.clone() {
                LayerData::Finite(t) => t,
                _ => vec![],
            };
            if is_move_valid(sprite_transform.clone(), delta, tile_vec) {
                walkable.state = next_walkablestate;
                sprite_transform.translation += delta;
            } else {
                println!("Invalid move");
                return;
            }
        }
    }
    for (_, mut transform) in set.q1_mut().iter_mut() {
        transform.translation += delta;
    }
}

fn is_move_valid(transform: Transform, delta: Vec3, tile_vec: Vec<Vec<LayerTile>>) -> bool {
    let mut next_transform = transform.clone();
    next_transform.translation += delta;

    let (next_x, next_y): (usize, usize) = (
        next_transform.translation.x as usize,
        next_transform.translation.y as usize,
    );
    println!("{:?}", (next_x, next_y));
    tile_vec[next_x][next_y].gid == 0
}
