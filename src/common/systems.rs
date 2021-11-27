use super::components::*;
use bevy::{ecs::component::Component, prelude::*};

pub fn animate_spriteplex<S: Component + Spriteplex>(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut S,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut animation_timer, mut spriteplex_box, mut sprite, texture_atlas_handle) in
        query.iter_mut()
    {
        animation_timer.timer.tick(time.delta());
        if animation_timer.force_update || animation_timer.timer.finished() {
            if let Some(_texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                let animation_strip = spriteplex_box.get_animation_strip();
                sprite.index = animation_strip.get_frame();
                sprite.flip_x = animation_strip.flip_x;
                sprite.flip_y = animation_strip.flip_y;
                spriteplex_box.next_frame();
            }
            if animation_timer.force_update {
                animation_timer.force_update = false;
            }
        }
    }
}

pub fn hot_reload_assets(asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
}
