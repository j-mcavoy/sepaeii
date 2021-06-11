use super::components::map_layers::*;
use super::components::*;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_tiled_prototype::Map;
use tiled::LayerTile;

const VELOCITY: f32 = 300.;

const UP_OFFSETS: [(f32, f32, f32); 2] = [(-8., 0., 0.), (8., 0., 0.)];
const DOWN_OFFSETS: [(f32, f32, f32); 2] = [(-8., -10., 0.), (8., -10., 0.)];
const LEFT_OFFSETS: [(f32, f32, f32); 2] = [(-8., 0., 0.), (-8., -10., 0.)];
const RIGHT_OFFSETS: [(f32, f32, f32); 2] = [(8., 0., 0.), (8., -10., 0.)];
pub fn npc_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&NPC, &mut Walkable, &mut AnimationTimer)>,
) {
    let delta = VELOCITY * time.delta_seconds();
    let mut next = Vec3::ZERO;

    for (npc, mut walkable, mut animation_timer) in query.iter_mut() {
        let curr_walkablestate = walkable.state;

        if curr_walkablestate != next_walkablestate {
            walkable.reset_animation_strip();
        }
        walkable.state = next_walkablestate;
    }
}
