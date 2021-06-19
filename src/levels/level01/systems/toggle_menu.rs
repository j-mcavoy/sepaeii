use crate::level01::states::Level01State;

use super::states::Level01State::*;
use bevy::prelude::*;

pub fn toggle_menu(
    mut app_state: ResMut<State<Level01State>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let next_state = match app_state.current() {
            Play => Menu,
            Menu => Play,
        };
        app_state.set(next_state).unwrap();
    }
}
