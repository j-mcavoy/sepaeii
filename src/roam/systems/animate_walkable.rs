use bevy::prelude::*;

use super::components::*;

pub fn animate_walkable(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut Walkable,
        &Transform,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut animation_timer, mut walkable, _transform, mut sprite, texture_atlas_handle) in
        query.single_mut()
    {
        animation_timer.0.tick(time.delta());
        if animation_timer.0.finished() {
            if let Some(_texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                let state = walkable.state;
                let animation_strip = match state {
                    WalkableState::StillUp => &mut walkable.still_up,
                    WalkableState::StillDown => &mut walkable.still_down,
                    WalkableState::StillLeft => &mut walkable.still_left,
                    WalkableState::StillRight => &mut walkable.still_right,
                    WalkableState::WalkUp => &mut walkable.walk_up,
                    WalkableState::WalkDown => &mut walkable.walk_down,
                    WalkableState::WalkLeft => &mut walkable.walk_left,
                    WalkableState::WalkRight => &mut walkable.walk_right,
                };
                animation_strip.next_frame();
                sprite.index = animation_strip.get_frame();
                sprite.flip_x = animation_strip.flip_x;
                sprite.flip_y = animation_strip.flip_y;
            }
        }
    }
}
