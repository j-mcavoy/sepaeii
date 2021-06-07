use super::components::map_layers::*;
use super::components::*;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_tiled_prototype::Map;
use tiled::LayerTile;

const VELOCITY: f32 = 300.;
pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut set: QuerySet<(
        Query<(
            &PandaMan,
            &mut Walkable,
            &mut Transform,
            &mut AnimationTimer,
            &mut BoxCollider,
        )>,
        Query<(&Camera, &mut Transform)>,
    )>,
    asset_server: Res<AssetServer>,
    maps_handle: Res<Assets<Map>>,
) {
    let mut delta = Vec3::ZERO;

    for (_pandaman, mut walkable, mut sprite_transform, _animation_timer, mut collider) in
        set.q0_mut().iter_mut()
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
                tiled::LayerData::Finite(t) => t,
                _ => vec![],
            };
            if is_move_valid(&collider, &tile_vec, delta) {
                walkable.state = next_walkablestate;
                sprite_transform.translation += delta;
                collider.origin += Vec2::new(delta.x, delta.y);
            } else {
                walkable.state = match curr_walkablestate {
                    WalkableState::WalkUp => WalkableState::StillUp,
                    WalkableState::WalkDown => WalkableState::StillDown,
                    WalkableState::WalkLeft => WalkableState::StillLeft,
                    WalkableState::WalkRight => WalkableState::StillRight,
                    _ => curr_walkablestate,
                };
                println!("Invalid move");
                delta = Vec3::ZERO;
            }
        }
    }
    for (_, mut transform) in set.q1_mut().iter_mut() {
        transform.translation += delta;
    }
}

fn is_move_valid(collider: &BoxCollider, tile_vec: &Vec<Vec<LayerTile>>, delta: Vec3) -> bool {
    let mut c = collider.clone();
    c.origin += (delta.x, delta.y).into();
    println!("Delta: {:?}", delta);
    let points = if delta.x > -0. {
        vec![coord2map_index(&c.ne()), coord2map_index(&c.se())]
    } else if delta.x < -0. {
        vec![coord2map_index(&c.nw()), coord2map_index(&c.sw())]
    } else if delta.y > -0. {
        vec![coord2map_index(&c.nw()), coord2map_index(&c.ne())]
    } else if delta.y < -0. {
        vec![coord2map_index(&c.sw()), coord2map_index(&c.se())]
    } else {
        return true;
    };

    println!("Points: {:?}", points);
    for point in points.iter() {
        if tile_vec[point.1][point.0].gid != 0 {
            println!("false");
            return false;
        }
    }
    return true;
}

pub fn coord2map_index(coord: &Vec2) -> (usize, usize) {
    ((coord.x / 32.) as usize, (-coord.y / 32.) as usize)
}
