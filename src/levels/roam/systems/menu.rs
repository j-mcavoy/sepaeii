use super::components::*;
use bevy::prelude::*;

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("setup_menu");
    //commands.spawn().insert_bundle(UiCameraBundle::default());

    commands
        .spawn()
        .insert(MainMenu {})
        .insert_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Relative,
                position: Rect {
                    ..Default::default()
                },
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                "Super Epic\nPanda Adventure\nExtreme II",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        });
}

pub fn destroy_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    println!("destroy_menu called");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
