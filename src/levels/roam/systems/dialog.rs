use super::super::components::*;
use super::map::coord2map_index;
use super::states::*;
use bevy::prelude::*;

pub fn npc_interactions(
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    pandaman_query: Query<(&Transform, &PandaManSpriteplex), With<PandaMan>>,
    dialog_query: Query<&Dialog>,
    mut npc_query: Query<
        (&Transform, &mut NPCSpriteplex, &mut AnimationTimer, &NPC),
        Without<PandaMan>,
    >,
    mut app_state: ResMut<State<RoamState>>,
) {
    if keyboard_input.just_pressed(KeyCode::C) && dialog_query.single().is_err() {
        keyboard_input.reset(KeyCode::C);
        let (pandaman_transform, player_spriteplex) = pandaman_query.single().unwrap();
        let px = pandaman_transform.translation.x;
        let py = pandaman_transform.translation.y;
        for (npc_transform, mut npc_spriteplex, mut animation_timer, npc) in npc_query.iter_mut() {
            let nx = npc_transform.translation.x;
            let ny = npc_transform.translation.y;

            let (px, py) = coord2map_index(&(px, py).into());
            let (nx, ny) = coord2map_index(&(nx, ny).into());
            let state = player_spriteplex.state;

            let next_state = if px == nx
                && py == ny + 1
                && matches!(state, PandaManState::StillUp | PandaManState::WalkUp)
            {
                println!("Down");
                Some(NPCState::StillDown)
            } else if px == nx
                && py == ny - 1
                && matches!(state, PandaManState::StillDown | PandaManState::WalkDown)
            {
                println!("up");
                Some(NPCState::StillUp)
            } else if px == nx + 1
                && py == ny
                && matches!(state, PandaManState::StillLeft | PandaManState::WalkLeft)
            {
                println!("right");
                Some(NPCState::StillRight)
            } else if px == nx - 1
                && py == ny
                && matches!(state, PandaManState::StillRight | PandaManState::WalkRight)
            {
                println!("left");
                Some(NPCState::StillLeft)
            } else {
                None
            };
            if next_state.is_some() {
                if next_state.unwrap() != npc_spriteplex.state {
                    npc_spriteplex.state = next_state.unwrap();
                    animation_timer.force_update = true;
                    animation_timer.timer.reset();
                }
                app_state.push(RoamState::Dialog).unwrap();
            }
        }
    }
}

pub fn dialog(
    mut dialog_query: Query<&mut Dialog>,
    mut text_query: Query<&mut Text, With<DialogUi>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<RoamState>>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        keyboard_input.reset(KeyCode::C);
        let mut dialog = dialog_query.single_mut().unwrap();
        if dialog.len() > 0 {
            let mut text = text_query.single_mut().unwrap();
            text.sections[0].value = dialog.pop_front().unwrap();
        } else {
            app_state.pop().unwrap();
            keyboard_input.reset(KeyCode::C);
        }
    }
}
pub fn setup_dialog(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(DialogUi);
    commands.spawn_bundle(DialogUiBundle::from((
        vec![
            "Hello".to_string(),
            "There".to_string(),
            "Pandaman".to_string(),
        ]
        .into(),
        font,
    )));
}
pub fn destroy_dialog(mut commands: Commands, query: Query<Entity, With<DialogUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
