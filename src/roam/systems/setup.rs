use bevy::prelude::*;
use bevy_tiled_prototype::TiledMapCenter;

use super::super::components::*;

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
    commands
        .spawn()
        .insert(Map)
        .insert_bundle(bevy_tiled_prototype::TiledMapBundle {
            map_asset: asset_server.load("levels/roam/map2.tmx"),
            center: TiledMapCenter(false),
            origin: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        });
    commands
        .spawn()
        .insert(PandaMan {})
        .insert(Transform::from_xyz(10., 10., 10.))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: pandaman,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Walkable {
            still_up: vec![7].into(),
            still_down: vec![1].into(),
            still_left: vec![4].into(),
            still_right: vec![4].into(),
            walk_up: vec![6, 7, 8, 7].into(),
            walk_down: vec![0, 1, 2, 1].into(),
            walk_left: AnimationStrip(vec![3, 4, 5, 4].into(), true, false),
            walk_right: vec![3, 4, 5, 4].into(),
            state: WalkableState::StillDown,
        })
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)));
}
