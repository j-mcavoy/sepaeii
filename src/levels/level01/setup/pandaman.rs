use crate::level01::components::map_layers::*;

use super::SPAWN_POINT;
use crate::common::components::*;
use bevy::prelude::*;

use super::super::components::*;

pub fn setup_pandaman(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let pandaman = TextureAtlas::from_grid(
        asset_server.load("levels/level01/sprites/marioPanda.png"),
        Vec2::new(17.0, 28.0),
        21,
        6,
    );
    let pandaman = texture_atlases.add(pandaman);

    commands
        .spawn()
        .insert(PandaMan {})
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: pandaman,
            transform: Transform::from_xyz(SPAWN_POINT.0, SPAWN_POINT.1, BACKGROUND as f32 + 10.1),
            ..Default::default()
        })
        // TODO: Insert Mario
        //.insert(Mario {
        //    still_left: AnimationStrip {
        //        sequence: vec![0],
        //        flip_x: true,
        //        ..Default::default()
        //    },
        //    still_right: vec![0].into(),
        //    jump: vec![4].into(),
        //    walk_left: AnimationStrip {
        //        sequence: vec![0],
        //        flip_x: true,
        //        ..Default::default()
        //    },
        //    walk_right: vec![0].into(),
        //    ..Default::default()
        //})
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)));
}
