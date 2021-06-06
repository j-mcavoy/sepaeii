use bevy::prelude::*;
use bevy_tiled_prototype::MapReadyEvent;

pub fn debug(mut event_rd: EventReader<MapReadyEvent>) {
    for _ in event_rd.iter() {
        println!("Map Ready!");
    }
}

pub fn debug_collision(query: Query<&BoxCollider>) {
    for collider in query.iter() {}
}
