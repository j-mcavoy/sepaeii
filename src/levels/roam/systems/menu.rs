use super::components::*;

use bevy::prelude::*;

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("setup_menu");
    let font = asset_server.load("fonts/FiraSans-Bold.ttf").clone_weak();
    commands
        .spawn()
        .insert(MainMenu {})
        .insert_bundle(UiCameraBundle::default())
        .insert_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(15.),
                    left: Val::Px(100.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Super Epic Panda Adventure\nExtreme II",
                TextStyle {
                    font,
                    font_size: 50.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..Default::default()
        });
}

pub fn destroy_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    println!("destroy_menu called");
    for entity in query.iter() {
        println!("entiry despwaned");
        commands.entity(entity).despawn_recursive();
    }
}
