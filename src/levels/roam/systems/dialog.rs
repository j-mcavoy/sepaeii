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

pub fn setup_dialog() {}
pub fn destroy_dialog() {}

pub fn dialog(
    mut commands: Commands,
    mut dialog_query: Query<&Dialog>,
    transform_query: Query<&Transform, With<PandaMan>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<State<RoamState>>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        keyboard_input.reset(KeyCode::C);
        let result = app_state.pop();
        println!("pop dialog {:?}", result);
    }
    //for mut dialog in dialog_query.single_mut() {
    //    let transform = *transform_query.single().unwrap();
    //    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    //    commands.spawn_bundle(DialogUiBundle::from((dialog.clone(), transform, font)));
    //}
}
