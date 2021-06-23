use bevy::prelude::*;

use bevy_tiled_prototype::TiledMapCenter;

use super::super::components::*;
use super::SPAWN_POINT;
pub fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(Map)
        .insert_bundle(bevy_tiled_prototype::TiledMapBundle {
            map_asset: asset_server.load("levels/level01/lvlMario.tmx"),
            center: TiledMapCenter(false),
            origin: Transform::from_xyz(SPAWN_POINT.0, SPAWN_POINT.1, 0.),
            ..Default::default()
        });
}
