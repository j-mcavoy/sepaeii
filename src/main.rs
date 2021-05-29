use std::borrow::BorrowMut;

use bevy::{
    app::CoreStage::PreUpdate,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::Quat,
    prelude::Bundle,
    prelude::*,
    prelude::*,
    render::camera::Camera,
};
use bevy_tiled_prototype::{MapReadyEvent, TiledMapCenter};

//#[derive(Bundle, Clone)]
struct Character {
    //    #[bundle]
//    sprite: SpriteBundle,
}

struct Location(Vec2);
struct Animation(TextureAtlas);
struct Walkable {
    animation_still: TextureAtlas,
    animation_up: TextureAtlas,
    animation_down: TextureAtlas,
    animation_left: TextureAtlas,
    animation_right: TextureAtlas,
}

#[derive(Debug)]
enum Direction {
    Still,
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_tiled_prototype::TiledMapPlugin)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_startup_system(setup.system())
        .add_system(player_movement.system())
        .add_system(animate_sprite_system.system())
        // Needs to run before rendering to set texture atlas filter for new tiles- would be better to use states
        .add_system_to_stage(PreUpdate, set_texture_filters_to_nearest.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle: Handle<Texture> = asset_server.load("sprites/panda.png");
    let still = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaStillDown"),
        Vec2::new(29.0, 31.0),
        1,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(still);
    let still2 = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaStillDown"),
        Vec2::new(29.0, 31.0),
        1,
        1,
    );
    let up = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaWalkUp"),
        Vec2::new(29.0, 31.0),
        3,
        1,
    );
    let down = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaWalkDown"),
        Vec2::new(29.0, 31.0),
        3,
        1,
    );
    let left = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaWalkLeft"),
        Vec2::new(29.0, 31.0),
        3,
        1,
    );
    let right = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaWalkRight"),
        Vec2::new(29.0, 31.0),
        3,
        1,
    );

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::new(5., 5., 2.)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true));
    commands.spawn_bundle(bevy_tiled_prototype::TiledMapBundle {
        map_asset: asset_server.load("levels/roam/map2.tmx"),
        center: TiledMapCenter(false),
        origin: Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
        ..Default::default()
    });
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

const VELOCITY: f32 = 2.;
fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
    mut query2: Query<(&mut Character, &mut Walkable, &TextureAtlas)>,
) {
    for (_, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let scale = transform.scale.x;

        let mut dir = Direction::Still;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
            dir = Direction::Left
        } else if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            dir = Direction::Right
        } else if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            dir = Direction::Up
        } else if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
            dir = Direction::Down
        }

        for (mut character, mut walkable, mut texture_atlas) in query2.iter_mut() {
            println!("dir={:?}", dir);
            texture_atlas = match dir {
                Direction::Still => &walkable.animation_still,
                Direction::Up => &walkable.animation_up,
                Direction::Down => &walkable.animation_down,
                Direction::Left => &walkable.animation_left,
                Direction::Right => &walkable.animation_right,
            };
        }
        transform.translation += time.delta_seconds() * direction * VELOCITY * 1000.;
    }
}

fn set_texture_filters_to_nearest(
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
