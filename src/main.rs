use bevy::{
    app::CoreStage::PreUpdate, prelude::Bundle, prelude::*, render::camera::Camera,
    utils::tracing::field::debug,
};
use bevy_tiled_prototype::{MapReadyEvent, TiledMapCenter};

#[derive(Bundle)]
struct PandaMan {
    pub transform: Transform,
    pub walkable: Walkable,
}

#[derive(Debug)]
struct Walkable {
    pub still_up: Handle<TextureAtlas>,
    pub still_down: Handle<TextureAtlas>,
    pub still_left: Handle<TextureAtlas>,
    pub still_right: Handle<TextureAtlas>,
    pub walk_up: Handle<TextureAtlas>,
    pub walk_down: Handle<TextureAtlas>,
    pub walk_left: Handle<TextureAtlas>,
    pub walk_right: Handle<TextureAtlas>,
    pub state: WalkableState,
}

#[derive(Debug, Clone)]
enum WalkableState {
    StillUp,
    StillDown,
    StillLeft,
    StillRight,
    WalkUp,
    WalkDown,
    WalkLeft,
    WalkRight,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_tiled_prototype::TiledMapPlugin)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_startup_system(setup.system())
        .add_system(player_movement.system())
        .add_system(animate_sprite_system.system())
        .add_system_to_stage(PreUpdate, set_texture_filters_to_nearest.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let still_up = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaStillDown.png"),
        Vec2::new(29.0, 31.0),
        1,
        1,
    );
    let still_down = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaStillDown.png"),
        Vec2::new(29.0, 31.0),
        1,
        1,
    );
    let still_down2 = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaStillDown.png"),
        Vec2::new(29.0, 31.0),
        1,
        1,
    );
    let still_left = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaStillDown.png"),
        Vec2::new(29.0, 31.0),
        1,
        1,
    );
    let still_right = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaStillDown.png"),
        Vec2::new(29.0, 31.0),
        1,
        1,
    );
    let still_right2 = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaStillDown.png"),
        Vec2::new(29.0, 31.0),
        1,
        1,
    );
    let walk_down = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaStillDown.png"),
        Vec2::new(29.0, 31.0),
        4,
        1,
    );
    let walk_up = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaStillDown.png"),
        Vec2::new(29.0, 31.0),
        4,
        1,
    );
    let walk_left = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaStillDown.png"),
        Vec2::new(29.0, 31.0),
        4,
        1,
    );
    let walk_right = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaStillDown.png"),
        Vec2::new(29.0, 31.0),
        4,
        1,
    );
    let walk_right2 = TextureAtlas::from_grid(
        asset_server.load("sprites/pandaStillDown.png"),
        Vec2::new(29.0, 31.0),
        4,
        1,
    );

    let still_up = texture_atlases.add(still_up);
    let still_down = texture_atlases.add(still_down);
    let still_down2 = texture_atlases.add(still_down2);
    let still_left = texture_atlases.add(still_left);
    let still_right = texture_atlases.add(still_right);
    let still_right2 = texture_atlases.add(still_right2);
    let walk_up = texture_atlases.add(walk_up);
    let walk_down = texture_atlases.add(walk_down);
    let walk_left = texture_atlases.add(walk_left);
    let walk_right = texture_atlases.add(walk_right);
    let walk_right2 = texture_atlases.add(walk_right2);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(bevy_tiled_prototype::TiledMapBundle {
        map_asset: asset_server.load("levels/roam/map2.tmx"),
        center: TiledMapCenter(false),
        origin: Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
        ..Default::default()
    });
    commands
        .spawn()
        .insert(PandaMan {
            transform: Transform::from_xyz(10., 10., 0.),
            walkable: Walkable {
                state: WalkableState::StillRight,
                walk_up: walk_up.clone(),
                walk_down: walk_down.clone(),
                walk_left: walk_left.clone(),
                walk_right: walk_right.clone(),
                still_up: still_up.clone(),
                still_down: still_down.clone(),
                still_left: still_left.clone(),
                still_right: still_right.clone(),
            },
        })
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: walk_right2,
            transform: Transform::from_scale(Vec3::new(1., 1., 10.)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.2, true));
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &PandaMan,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (_, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                sprite.index = 0; //((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            }
        }
    }
}

const VELOCITY: f32 = 0.3;
fn player_movement(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PandaMan, &mut Handle<TextureAtlas>, &mut Transform)>,
) {
    for (mut pandaman, mut sprite_handle, mut sprite_transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
            pandaman.walkable.state = WalkableState::WalkLeft;
        } else if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            pandaman.walkable.state = WalkableState::WalkRight;
        } else if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            pandaman.walkable.state = WalkableState::WalkUp;
        } else if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
            pandaman.walkable.state = WalkableState::WalkDown;
        } else {
            pandaman.walkable.state = match pandaman.walkable.state {
                WalkableState::WalkUp => WalkableState::StillUp,
                WalkableState::WalkDown => WalkableState::StillDown,
                WalkableState::WalkLeft => WalkableState::StillLeft,
                WalkableState::WalkRight => WalkableState::StillRight,
                _ => pandaman.walkable.state.clone(),
            }
        }

        *sprite_handle = match pandaman.walkable.state {
            WalkableState::StillUp => pandaman.walkable.still_up.clone_weak(),
            WalkableState::StillDown => pandaman.walkable.still_down.clone_weak(),
            WalkableState::StillLeft => pandaman.walkable.still_left.clone_weak(),
            WalkableState::StillRight => pandaman.walkable.still_right.clone_weak(),
            WalkableState::WalkUp => pandaman.walkable.still_down.clone_weak(),
            WalkableState::WalkDown => pandaman.walkable.still_down.clone_weak(),
            WalkableState::WalkLeft => pandaman.walkable.still_left.clone_weak(),
            WalkableState::WalkRight => pandaman.walkable.still_right.clone_weak(),
        };

        sprite_transform.translation += time.delta_seconds() * direction * VELOCITY * 1000.;
        let mut transform = pandaman.transform;
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
