use bevy::prelude::*;

use super::components::*;

pub fn animate_character(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut Character,
        &Transform,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut animation_timer, mut walkable, _transform, mut sprite, texture_atlas_handle) in
        query.iter_mut()
    {
        animation_timer.0.tick(time.delta());
        if animation_timer.0.finished() {
            if let Some(_texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                let state = walkable.state;
                let animation_strip = match state {
                    CharacterState::StillLeft => &mut walkable.still_left,
                    CharacterState::StillRight => &mut walkable.still_right,
                    CharacterState::WalkLeft => &mut walkable.walk_left,
                    CharacterState::WalkRight => &mut walkable.walk_right,
                    CharacterState::Jump => &mut walkable.jump,
                };
                animation_strip.next_frame();
                sprite.index = animation_strip.get_frame();
                sprite.flip_x = animation_strip.flip_x;
                sprite.flip_y = animation_strip.flip_y;
            }
        }
    }
}
