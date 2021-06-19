use crate::common::components::*;
use crate::roam::components::map_layers::*;

use bevy::prelude::*;

use super::super::components::*;

pub fn setup_npc(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let npc1_spawn = Vec2::new(16., -7.) + Vec2::new(32., -21.) * 32.;
    let npc2_spawn = Vec2::new(16., -7.) + Vec2::new(9., -94.) * 32.;
    let npc3_spawn = Vec2::new(16., -7.) + Vec2::new(93., -92.) * 32.;
    let npc4_spawn = Vec2::new(16., -7.) + Vec2::new(95., -91.) * 32.;

    let atlas = TextureAtlas::from_grid(
        asset_server.load("levels/roam/sprites/chars.png"),
        Vec2::new(32.0, 32.0),
        4,
        3,
    );
    let atlas_handle = texture_atlases.add(atlas);

    commands
        .spawn()
        .insert(NPC {})
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: atlas_handle.clone(),
            transform: Transform::from_xyz(npc1_spawn.x, npc1_spawn.y, OBJECTS as f32 + 0.1),
            ..Default::default()
        })
        .insert(NPCSpriteplex {
            still_up: vec![8].into(),
            still_down: vec![0].into(),
            still_left: AnimationStrip {
                sequence: vec![4],
                flip_x: true,
                ..Default::default()
            },
            still_right: vec![4].into(),
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(1.0, true)));

    commands
        .spawn()
        .insert(NPC {})
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: atlas_handle.clone(),
            transform: Transform::from_xyz(npc2_spawn.x, npc2_spawn.y, GROUND2 as f32 + 0.1),
            ..Default::default()
        })
        .insert(NPCSpriteplex {
            still_up: vec![9].into(),
            still_down: vec![1].into(),
            still_left: AnimationStrip {
                sequence: vec![5],
                flip_x: true,
                ..Default::default()
            },
            still_right: vec![5].into(),
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(1.0, true)));

    commands
        .spawn()
        .insert(NPC {})
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: atlas_handle.clone(),
            transform: Transform::from_xyz(npc3_spawn.x, npc3_spawn.y, GROUND2 as f32 + 0.1),
            ..Default::default()
        })
        .insert(NPCSpriteplex {
            still_up: vec![10].into(),
            still_down: vec![2].into(),
            still_left: AnimationStrip {
                sequence: vec![6],
                flip_x: true,
                ..Default::default()
            },
            still_right: vec![6].into(),
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(1.0, true)));

    commands
        .spawn()
        .insert(NPC {})
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: atlas_handle.clone(),
            transform: Transform::from_xyz(npc4_spawn.x, npc4_spawn.y, GROUND2 as f32 + 0.1),
            ..Default::default()
        })
        .insert(NPCSpriteplex {
            still_up: vec![11].into(),
            still_down: vec![3].into(),
            still_left: AnimationStrip {
                sequence: vec![7],
                flip_x: true,
                ..Default::default()
            },
            still_right: vec![7].into(),
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(1.0, true)));
}
