use crate::level01::components::map_layers::*;

use crate::common::components::*;
use crate::levels::level01::setup::*;
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
    let pandaman_atlas = texture_atlases.add(pandaman);

    commands
        .spawn()
        .insert(Mario::default())
        .insert(MarioSpriteplex {
            small_mario_still_right: vec![21].into(),
            small_mario_still_left: AnimationStrip {
                sequence: vec![22],
                flip_x: true,
                ..Default::default()
            },
            small_mario_walk_right: vec![23, 24, 25].into(),
            small_mario_walk_left: AnimationStrip {
                sequence: vec![23, 24, 25],
                flip_x: true,
                ..Default::default()
            },
            small_mario_crouch_right: vec![28].into(),
            small_mario_crouch_left: AnimationStrip {
                sequence: vec![28],
                flip_x: true,
                ..Default::default()
            },
            small_mario_jump: vec![26].into(),
            big_mario_still_right: vec![0].into(),
            big_mario_still_left: AnimationStrip {
                sequence: vec![0],
                flip_x: true,
                ..Default::default()
            },
            big_mario_walk_right: vec![1, 2, 3].into(),
            big_mario_walk_left: AnimationStrip {
                sequence: vec![1, 2, 3],
                flip_x: true,
                ..Default::default()
            },
            big_mario_crouch_right: vec![6].into(),
            big_mario_crouch_left: AnimationStrip {
                sequence: vec![6],
                flip_x: true,
                ..Default::default()
            },
            big_mario_jump: vec![5].into(),
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
            texture_atlas: pandaman_atlas,
            transform: Transform::from_xyz(
                0. * TILE_WIDTH,
                -1. * TILE_HEIGHT,
                BACKGROUND as f32 + 10.1,
            ),
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)));
}
