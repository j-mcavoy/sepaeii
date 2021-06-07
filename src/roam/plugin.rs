use bevy::prelude::*;

use super::states::RoamState;
use super::systems::*;

pub struct RoamPlugin;

impl Plugin for RoamPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(DefaultPlugins)
            .add_system(bevy::input::system::exit_on_esc_system.system())
            .add_state(RoamState::Menu)
            .add_plugin(bevy_tiled_prototype::TiledMapPlugin)
            //.add_system_set_to_stage(
            //    PreUpdate,
            //    SystemSet::on_update(RoamState::Play).with_system(tile_interpolation.system()),
            //)
            .add_startup_system(setup.system())
            .add_system_set(
                SystemSet::on_update(RoamState::Play)
                    .with_system(player_movement.system())
                    .with_system(animate_walkable.system()),
            )
            .add_system(toggle_menu.system())
            .add_system_set(
                SystemSet::on_update(RoamState::Play)
                    .with_system(map.system())
                    .with_system(debug.system())
                    .with_system(debug_movement.system()),
            )
            .add_system_set(
                SystemSet::on_update(RoamState::Play).with_system(tile_interpolation.system()),
            )
            .add_system_set(SystemSet::on_enter(RoamState::Menu).with_system(setup_menu.system()))
            .add_system_set(SystemSet::on_exit(RoamState::Menu).with_system(destroy_menu.system()));
    }
}
