
use bevy::prelude::*;
use bevy_tiled_prototype::Map;
use bevy_tiled_prototype::MapReadyEvent;



pub fn map(_asset_server: Res<AssetServer>, _maps_handle: Res<Assets<Map>>) {}

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
