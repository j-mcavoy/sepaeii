use bevy::prelude::*;

use super::components::*;
use super::setup::*;
use super::states::Level01State::*;
use super::systems::*;
use crate::common::systems::*;

pub struct Level01Plugin;

impl Plugin for Level01Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(DefaultPlugins)
            .add_system(bevy::input::system::exit_on_esc_system.system())
            .add_plugin(bevy_tiled_prototype::TiledMapPlugin)
            .add_state(Menu)
            .add_system(bevy::input::system::exit_on_esc_system.system())
            .add_system(toggle_menu.system())
            .add_startup_system(setup_camera.system())
            .add_startup_system(setup_map.system())
            .add_startup_system(setup_mario.system())
            .add_system_set(SystemSet::on_enter(Menu).with_system(setup_menu.system()))
            .add_system_set(SystemSet::on_exit(Menu).with_system(destroy_menu.system()))
            .add_system_set(
                SystemSet::on_update(Play)
                    .with_system(player_movement.system())
                    .with_system(camera_movement.system())
                    .with_system(animate_spriteplex::<MarioSpriteplex>.system()),
            );
    }
}
