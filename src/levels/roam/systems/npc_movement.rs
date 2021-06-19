use super::components::*;
use crate::common::components::*;
use bevy::prelude::*;

pub fn npc_movement(mut query: Query<(&NPC, &mut NPCSpriteplex, &AnimationTimer)>) {
    let _next = Vec3::ZERO;

    for (_npc, mut npc_spriteplex, animation_timer) in query.iter_mut() {
        if animation_timer.0.finished() {
            npc_spriteplex.state = match npc_spriteplex.state {
                NPCState::StillUp => NPCState::StillLeft,
                NPCState::StillDown => NPCState::StillRight,
                NPCState::StillLeft => NPCState::StillDown,
                NPCState::StillRight => NPCState::StillUp,
                _ => NPCState::StillDown,
            };
        }
    }
}
