use super::components::*;
use crate::common::components::*;
use bevy::prelude::*;
use bevy_tiled_prototype::MapReadyEvent;

pub fn debug(mut event_rd: EventReader<MapReadyEvent>) {
    for _ in event_rd.iter() {
        println!("Map Ready!");
    }
}
pub fn debug_movement(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&PandaMan, &Transform, &BoxCollider)>,
) {
    if keyboard_input.is_changed() {
        for (_, _transform, _box_collider) in query.iter() {
            //println!("Pandaman Transform: {:?}", &transform.translation);
            //println!("Box Collider Origin: {:?}", &box_collider.origin);
            //println!(
            //    "Map Location: {:?}",
            //    super::player_movement::coord2map_index(&Vec2::new(
            //        transform.translation.x,
            //        transform.translation.y
            //    ))
            //);
        }
    }
}
