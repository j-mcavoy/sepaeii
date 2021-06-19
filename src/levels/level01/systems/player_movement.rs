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
        Query<(&PandaMan, &mut Mario, &mut Transform, &mut AnimationTimer)>,
        Query<(&Camera, &mut Transform)>,
        Query<(&NPC, &Sprite)>,
    )>,
    maps_handle: Res<Assets<Map>>,
) {
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
