use bevy::prelude::*;
use bevy_tiled_prototype::Map;
use bevy_tiled_prototype::MapReadyEvent;
use bevy_tiled_prototype::TiledMapBundle;

pub fn map(maps: Res<Assets<Map>>) {
    //for map in maps.iter() {
    //    println!("{:?}", map.1.layers.len());
    //    for a in map.1.layers {
    //        for b in a.tileset_layers {
    //            b.chunks.
    //        }
    //    }
    //}
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
