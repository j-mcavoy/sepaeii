use super::components::map_layers::*;
use bevy::prelude::*;
use bevy_tiled_prototype::Map;
use bevy_tiled_prototype::MapReadyEvent;
use bevy_tiled_prototype::TiledMapBundle;
use tiled::LayerData;

pub fn map(asset_server: Res<AssetServer>, maps_handle: Res<Assets<Map>>) {
    for (map_handle, map) in maps_handle.iter() {
        let layer = map.map.layers.get(OBJECTS).unwrap();
        println!("{:?}", layer.name);
        let tile_vec = match layer.tiles.clone() {
            LayerData::Finite(t) => t,
            _ => vec![],
        };
        println!("{:?}", tile_vec[20][20].gid != 0);
    }
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
