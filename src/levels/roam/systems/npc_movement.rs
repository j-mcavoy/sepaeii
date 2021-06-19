use super::components::*;
use crate::common::components::*;
use bevy::prelude::*;

pub fn npc_movement(mut query: Query<(&NPC, &mut PandaManSprux, &AnimationTimer)>) {
    let _next = Vec3::ZERO;

    for (_npc, mut walkable, animation_timer) in query.iter_mut() {
        if animation_timer.0.finished() {
            walkable.state = match walkable.state {
                WalkableState::StillUp => WalkableState::StillLeft,
                WalkableState::StillDown => WalkableState::StillRight,
                WalkableState::StillLeft => WalkableState::StillDown,
                WalkableState::StillRight => WalkableState::StillUp,
                _ => WalkableState::StillDown,
            };
        }
    }
}
