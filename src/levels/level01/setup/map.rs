use bevy::prelude::*;



use super::super::components::*;

pub fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ground_handle = asset_server.load("assets/levels/level01/map/ground.png");
    commands.spawn().insert(Ground).insert_bundle(SpriteBundle {
        material: materials.add(ground_handle.into()),
        transform: Transform {
            translation: Vec3::new(0.0, -256.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
