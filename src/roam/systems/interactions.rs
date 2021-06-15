use super::components::*;
use bevy::prelude::*;



pub fn interactions(
    keyboard_input: Res<Input<KeyCode>>,
    query: QuerySet<(Query<(&PandaMan, &Transform)>, Query<(&NPC, &Transform)>)>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        for (_pandaman, _transform) in query.q0().iter() {
            for (_npc, _npc_transform) in query.q1().iter() {}
        }
    }
}
