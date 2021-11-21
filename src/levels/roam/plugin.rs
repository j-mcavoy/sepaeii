use bevy::prelude::*;

use super::components::*;
use super::setup::*;
use super::states::RoamState;
use super::systems::*;
use crate::common::systems::*;

pub struct RoamPlugin;

impl Plugin for RoamPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(DefaultPlugins)
            .add_system(bevy::input::system::exit_on_esc_system.system())
            .add_plugin(bevy_tiled_prototype::TiledMapPlugin)
            .add_state(RoamState::Menu)
            .add_system(bevy::input::system::exit_on_esc_system.system())
            .add_startup_system(hot_reload_assets.system())
            .add_system(toggle_menu.system())
            .add_startup_system(setup_camera.system())
            .add_startup_system(setup_map.system())
            .add_startup_system(setup_pandaman.system())
            .add_startup_system(setup_npc.system())
            .add_system_set(SystemSet::on_enter(RoamState::Menu).with_system(setup_menu.system()))
            .add_system_set(SystemSet::on_exit(RoamState::Menu).with_system(destroy_menu.system()))
            .add_system_set(
                SystemSet::on_update(RoamState::Play)
                    .with_system(animate_spriteplex::<PandaManSpriteplex>.system())
                    .with_system(animate_spriteplex::<NPCSpriteplex>.system())
                    .with_system(debug.system())
                    .with_system(npc_movement.system())
                    .with_system(player_movement.system())
                    .with_system(tile_interpolation.system()),
            );
    }
}
