use super::super::components::*;
use super::components::map_layers::*;
use super::components::SPAWN_POINT;
use super::map;
use crate::common::components::*;
use bevy::prelude::*;
use bevy_tiled_prototype::Map;
use tiled::LayerTile;

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
            transform: Transform::from_xyz(SPAWN_POINT.0, SPAWN_POINT.1, GROUND2 as f32 + 0.2),
            ..Default::default()
        })
        .insert(PandaManSpriteplex {
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
                sequence: vec![4, 3, 4, 5],
                flip_x: true,
                ..Default::default()
            },
            walk_right: vec![4, 3, 4, 5].into(),
            state: PandaManState::StillDown,
        })
        .insert(AnimationTimer::from_timer(Timer::from_seconds(0.2, true)));
}

const VELOCITY: f32 = 300.;

const UP_OFFSETS: [(f32, f32, f32); 2] = [(-8., 0., 0.), (8., 0., 0.)];
const DOWN_OFFSETS: [(f32, f32, f32); 2] = [(-8., -20., 0.), (8., -20., 0.)];
const LEFT_OFFSETS: [(f32, f32, f32); 2] = [(-8., 0., 0.), (-8., -20., 0.)];
const RIGHT_OFFSETS: [(f32, f32, f32); 2] = [(8., 0., 0.), (8., -20., 0.)];

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &PandaMan,
        &mut PandaManSpriteplex,
        &mut Transform,
        &mut AnimationTimer,
    )>,
    maps_handle: Res<Assets<Map>>,
) {
    let delta = VELOCITY * time.delta_seconds();

    let (_pandaman, mut walkable, mut sprite_transform, mut animation_timer) =
        query.single_mut().unwrap();

    let mut next = Vec3::ZERO;
    for (_, map) in maps_handle.iter() {
        let objects_layer = map.map.layers.get(OBJECTS).unwrap();
        let object_layer = match &objects_layer.tiles {
            tiled::LayerData::Finite(t) => t,
            _ => panic!("invalid map layer"),
        };
        let ground2_layer = map.map.layers.get(GROUND2).unwrap();
        let ground2_layer = match &ground2_layer.tiles {
            tiled::LayerData::Finite(t) => t,
            _ => panic!("invalid map layer"),
        };

        let curr_walkablestate = walkable.state;
        let (next_walkablestate, offsets, temp_next) =
            if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
                (
                    PandaManState::WalkUp,
                    Some(UP_OFFSETS),
                    Some(delta * Vec3::Y),
                )
            } else if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
                (
                    PandaManState::WalkLeft,
                    Some(LEFT_OFFSETS),
                    Some(delta * -Vec3::X),
                )
            } else if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
                (
                    PandaManState::WalkDown,
                    Some(DOWN_OFFSETS),
                    Some(delta * -Vec3::Y),
                )
            } else if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
                (
                    PandaManState::WalkRight,
                    Some(RIGHT_OFFSETS),
                    Some(delta * Vec3::X),
                )
            } else {
                (
                    match curr_walkablestate {
                        PandaManState::WalkUp => PandaManState::StillUp,
                        PandaManState::WalkDown => PandaManState::StillDown,
                        PandaManState::WalkLeft => PandaManState::StillLeft,
                        PandaManState::WalkRight => PandaManState::StillRight,
                        _ => curr_walkablestate,
                    },
                    None,
                    None,
                )
            };
        if offsets.is_some()
            && are_spaces_valid(
                offsets
                    .unwrap()
                    .iter()
                    .map(|offset| {
                        sprite_transform.translation + temp_next.unwrap() + (*offset).into()
                    })
                    .collect::<Vec<Vec3>>(),
                &object_layer,
                &ground2_layer,
            )
        {
            next = temp_next.unwrap()
        }

        sprite_transform.translation += next;

        if curr_walkablestate != next_walkablestate {
            walkable.reset_animation_strip();
            animation_timer.force_update = true;
        }
        walkable.state = next_walkablestate;
    }
}

fn are_spaces_valid(
    player_coords: Vec<Vec3>,
    object_layer: &Vec<Vec<LayerTile>>,
    ground2_layer: &Vec<Vec<LayerTile>>,
) -> bool {
    print!("player_coords: {:?}", player_coords);
    let out = player_coords
        .iter()
        .inspect(|x| println!("{:?}", x))
        .map(|vec3| map::coord2map_index(&(vec3.x, vec3.y).into()))
        .inspect(|x| println!("{:?}", x))
        .all(|(x, y)| ground2_layer[y][x].gid != 0 || object_layer[y][x].gid == 0);
    if !out {
        println!("INVALID MOVE");
    }
    out
}
