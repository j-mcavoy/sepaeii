
use super::components::*;
use bevy::prelude::*;




const VELOCITY: f32 = 300.;

const UP_OFFSETS: [(f32, f32, f32); 2] = [(-8., 0., 0.), (8., 0., 0.)];
const DOWN_OFFSETS: [(f32, f32, f32); 2] = [(-8., -10., 0.), (8., -10., 0.)];
const LEFT_OFFSETS: [(f32, f32, f32); 2] = [(-8., 0., 0.), (-8., -10., 0.)];
const RIGHT_OFFSETS: [(f32, f32, f32); 2] = [(8., 0., 0.), (8., -10., 0.)];
pub fn npc_movement(
    time: Res<Time>,
    _keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&NPC, &mut Walkable, &mut AnimationTimer)>,
) {
    let _delta = VELOCITY * time.delta_seconds();
    let _next = Vec3::ZERO;

    for (_npc, walkable, _animation_timer) in query.iter_mut() {
        let _curr_walkablestate = walkable.state;
    }
}
