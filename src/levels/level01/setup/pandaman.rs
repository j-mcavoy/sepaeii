use crate::level01::components::map_layers::*;

use super::SPAWN_POINT;
use crate::common::components::*;
use bevy::prelude::*;

use super::super::components::*;

pub fn setup_mario(
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
        .insert(Mario::default())
        .insert(MarioSpriteplex {
            small_mario_jump: vec![0].into(),
            small_mario_still_left: vec![0].into(),
            small_mario_still_right: vec![0].into(),
            small_mario_walk_left: vec![0].into(),
            small_mario_walk_right: vec![0].into(),
            small_mario_crouch_right: vec![0].into(),
            small_mario_crouch_left: vec![0].into(),
            small_star_jump: vec![0].into(),
            small_star_still_left: vec![0].into(),
            small_star_still_right: vec![0].into(),
            small_star_walk_left: vec![0].into(),
            small_star_walk_right: vec![0].into(),
            small_star_crouch: vec![0].into(),
            small_fire_power_still_right: vec![0].into(),
            small_fire_power_walk_left: vec![0].into(),
            small_fire_power_walk_right: vec![0].into(),
            small_fire_power_crouch_right: vec![0].into(),
            small_fire_power_crouch_left: vec![0].into(),
            big_mario_jump: vec![0].into(),
            big_mario_still_left: vec![0].into(),
            big_mario_still_right: vec![0].into(),
            big_mario_walk_left: vec![0].into(),
            big_mario_walk_right: vec![0].into(),
            big_mario_crouch_right: vec![0].into(),
            big_mario_crouch_left: vec![0].into(),
            big_star_jump: vec![0].into(),
            big_star_still_left: vec![0].into(),
            big_star_still_right: vec![0].into(),
            big_star_walk_left: vec![0].into(),
            big_star_walk_right: vec![0].into(),
            big_star_crouch_right: vec![0].into(),
            big_star_crouch_left: vec![0].into(),
            big_fire_power_jump: vec![0].into(),
            big_fire_power_still_left: vec![0].into(),
            big_fire_power_still_right: vec![0].into(),
            big_fire_power_walk_left: vec![0].into(),
            big_fire_power_walk_right: vec![0].into(),
            big_fire_power_crouch_right: vec![0].into(),
            big_fire_power_crouch_left: vec![0].into(),
            state: MarioState::SmallMarioStillRight,
        })
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
