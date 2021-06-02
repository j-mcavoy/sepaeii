use super::states::RoamState;
use bevy::prelude::*;

pub fn toggle_menu(mut app_state: ResMut<State<RoamState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let next_state = match app_state.current() {
            RoamState::Play => RoamState::Menu,
            RoamState::Menu => RoamState::Play,
        };
        app_state.set(next_state).unwrap();
    }
}
