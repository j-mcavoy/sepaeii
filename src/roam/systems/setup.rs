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

    let spawn_point = Vec2::new(35., -20.) * 32.;

    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_xyz(spawn_point.x, spawn_point.y, 100.),
        ..OrthographicCameraBundle::new_2d()
    });
    commands
        .spawn()
        .insert(Map)
        .insert_bundle(bevy_tiled_prototype::TiledMapBundle {
            map_asset: asset_server.load("levels/roam/map.tmx"),
            center: TiledMapCenter(false),
            origin: Transform::from_xyz(0., 0., 0.),
            debug_config: bevy_tiled_prototype::DebugConfig {
                enabled: true,
                ..Default::default()
            },
            ..Default::default()
        });
    commands
        .spawn()
        .insert(PandaMan {})
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: pandaman,
            transform: Transform::from_xyz(spawn_point.x, spawn_point.y, 3.1),
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
        .insert(BoxCollider {
            width: 21.,
            height: 29.,
            origin: spawn_point + Vec2::new(-2.0, -2.0),
        })
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)));
}
