use super::super::components::*;
use bevy::prelude::*;
use bevy_tiled_prototype::MapReadyEvent;
use bevy_tiled_prototype::TiledMapCenter;

pub fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(Map)
        .insert_bundle(bevy_tiled_prototype::TiledMapBundle {
            map_asset: asset_server.load("levels/roam/map.tmx"),
            center: TiledMapCenter(false),
            origin: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        });
}

pub fn tile_interpolation(
    mut map_ready_events: EventReader<MapReadyEvent>,
    mut textures: ResMut<Assets<Texture>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    // quick and dirty, run this for all textures every time a map is created/modified
    if map_ready_events.iter().count() > 0 {
        for (_, atlas) in texture_atlases.iter() {
            if let Some(texture) = textures.get_mut(atlas.texture.clone()) {
                texture.sampler.min_filter = bevy::render::texture::FilterMode::Nearest;
            }
        }
    }
}
pub fn coord2map_index(coord: &Vec2) -> (usize, usize) {
    ((coord.x / 32.) as usize, (-coord.y / 32.) as usize)
}
