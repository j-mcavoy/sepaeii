use crate::roam::components::map_layers::*;

use super::SPAWN_POINT;
use bevy::prelude::*;

use super::super::components::*;

pub fn setup_pandaman(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let pandaman = TextureAtlas::from_grid(
        asset_server.load("levels/roam/sprites/panda.png"),
        Vec2::new(29.0, 31.0),
        3,
        3,
    );
    let pandaman = texture_atlases.add(pandaman);

    commands
        .spawn()
        .insert(PandaMan {})
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: pandaman,
            transform: Transform::from_xyz(SPAWN_POINT.0, SPAWN_POINT.1, GROUND2 as f32 + 0.1),
            ..Default::default()
        })
        .insert(Walkable {
            still_up: vec![7].into(),
            still_down: vec![1].into(),
            still_left: AnimationStrip {
                sequence: vec![4],
                flip_x: true,
                ..Default::default()
            },
            still_right: vec![4].into(),
            walk_up: vec![6, 7, 8, 7].into(),
            walk_down: vec![0, 1, 2, 1].into(),
            walk_left: AnimationStrip {
                sequence: vec![3, 4, 5, 4],
                flip_x: true,
                ..Default::default()
            },
            walk_right: vec![3, 4, 5, 4].into(),
            state: WalkableState::StillDown,
        })
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)));
}
