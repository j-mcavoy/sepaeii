use bevy::{app::CoreStage::PreUpdate, prelude::*};
use bevy_tiled_prototype::{MapReadyEvent, TiledMapCenter};
use std::collections::VecDeque;

use super::components::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let pandaman = TextureAtlas::from_grid(
        asset_server.load("sprites/panda.png"),
        Vec2::new(29.0, 31.0),
        3,
        3,
    );
    let pandaman = texture_atlases.add(pandaman);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(bevy_tiled_prototype::TiledMapBundle {
        map_asset: asset_server.load("levels/roam/map2.tmx"),
        center: TiledMapCenter(false),
        origin: Transform::from_scale(Vec3::splat(1.0)),
        ..Default::default()
    });
    commands
        .spawn()
        .insert(PandaMan {})
        .insert(Transform::from_xyz(10., 10., 0.))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: pandaman,
            transform: Transform::from_scale(Vec3::new(1., 1., 10.)),
            ..Default::default()
        })
        .insert(Walkable {
            still_up: vec![7].into(),
            still_down: vec![1].into(),
            still_left: vec![4].into(),
            still_right: vec![4].into(),
            walk_up: vec![6, 7, 8].into(),
            walk_down: vec![0, 1, 2].into(),
            walk_left: vec![3, 4, 5].into(),
            walk_right: vec![3, 4, 5].into(),
            state: WalkableState::StillDown,
        })
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)));
}

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
                let queue: &mut VecDeque<u32> = match state {
                    WalkableState::StillUp => &mut walkable.still_up,
                    WalkableState::StillDown => &mut walkable.still_down,
                    WalkableState::StillLeft => &mut walkable.still_left,
                    WalkableState::StillRight => &mut walkable.still_right,
                    WalkableState::WalkUp => &mut walkable.walk_up,
                    WalkableState::WalkDown => &mut walkable.walk_down,
                    WalkableState::WalkLeft => &mut walkable.walk_left,
                    WalkableState::WalkRight => &mut walkable.walk_right,
                };
                sprite.index = queue[0];
                queue.rotate_right(1);
            }
        }
    }
}

const VELOCITY: f32 = 0.1;
pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PandaMan, &mut Walkable, &mut Transform)>,
) {
    for (_pandaman, mut walkable, mut sprite_transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
            walkable.state = WalkableState::WalkLeft;
        } else if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            walkable.state = WalkableState::WalkRight;
        } else if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            walkable.state = WalkableState::WalkUp;
        } else if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
            walkable.state = WalkableState::WalkDown;
        } else {
            walkable.state = match walkable.state {
                WalkableState::WalkUp => WalkableState::StillUp,
                WalkableState::WalkDown => WalkableState::StillDown,
                WalkableState::WalkLeft => WalkableState::StillLeft,
                WalkableState::WalkRight => WalkableState::StillRight,
                _ => walkable.state.clone(),
            }
        }

        sprite_transform.translation += time.delta_seconds() * direction * VELOCITY * 1000.;
    }
}

pub fn set_texture_filters_to_nearest(
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
