use bevy::prelude::*;

use bevy_tiled_prototype::TiledMapCenter;

use super::super::components::*;

pub fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(Map)
        .insert_bundle(bevy_tiled_prototype::TiledMapBundle {
            map_asset: asset_server.load("levels/level01/lvlMario.tmx"),
            center: TiledMapCenter(false),
            origin: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        });
}
