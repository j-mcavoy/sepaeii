use super::components::*;
use super::states::RoamState::{self, *};
use bevy::ecs::schedule::StateError::*;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};
use rand::Rng;

pub fn toggle_menu(mut app_state: ResMut<State<RoamState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if app_state.current() == &Menu {
            match app_state.pop() {
                Ok(_) => {}
                Err(StackEmpty) => {
                    app_state.set(Play).unwrap();
                }
                Err(AlreadyInState) => {}
                Err(StateAlreadyQueued) => {}
            }
        } else {
            app_state.push(Menu).unwrap();
        }
    }
}
pub fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    audio: Res<Audio>,
) {
    let mut rng = rand::thread_rng();
    let music_no = rng.gen_range(1u8..9u8);
    audio.play_looped_with_intro_in_channel(
        asset_server.load(format!("music/Metal{:02}Intro.ogg", music_no).as_str()),
        asset_server.load(format!("music/Metal{:02}Loop.ogg", music_no).as_str()),
        &AudioChannel::new("menu-music".to_owned()),
    );
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(MainMenu);
    // ui camera
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            // bevy logo (image)
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(500.0), Val::Auto),
                    ..Default::default()
                },
                material: materials.add(asset_server.load("images/title.png").into()),
                ..Default::default()
            });
        })
        .insert(MainMenu);
}
pub fn destroy_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenu>>,
    audio: Res<Audio>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    audio.stop_channel(&AudioChannel::new("menu-music".to_owned()));
}
