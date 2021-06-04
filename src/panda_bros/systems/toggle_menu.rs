use super::states::RoamState;
use bevy::prelude::*;

pub fn toggle_menu(mut app_state: ResMut<State<RoamState>>, keyboard_input: Res<Input<KeyCode>>) {
    let mut next_state;
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state = match app_state.current() {
            RoamState::Menu => RoamState::Play,
            _ => RoamState::Menu,
        };
    } else if keyboard_input.just_pressed(KeyCode::Enter) {
        next_state = match app_state.current() {
            PandaBrosState::Play => PandaBrosState::Menu,
            PandaBrosState::Menu => PandaBrosState::RoamState::PandaBrosState,
        };
        app_state.set(next_state).unwrap();
    }
}
