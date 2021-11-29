use super::components::*;
use super::states::RoamState;
use super::systems::*;
use crate::common::systems::*;
use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

pub struct RoamPlugin;

impl Plugin for RoamPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(AudioPlugin)
            .add_plugin(bevy_tiled_prototype::TiledMapPlugin)
            .add_startup_system(hot_reload_assets.system())
            .add_startup_system(setup_camera.system())
            .add_startup_system(setup_map.system())
            .add_startup_system(setup_pandaman.system())
            .add_startup_system(setup_npc.system())
            .add_system(toggle_menu.system().label("menu"))
            .add_system(bevy::input::system::exit_on_esc_system.system())
            .add_system_set(SystemSet::on_enter(RoamState::Menu).with_system(setup_menu.system()))
            .add_system_set(SystemSet::on_exit(RoamState::Menu).with_system(destroy_menu.system()))
            .add_system_set(SystemSet::on_update(RoamState::Dialog).with_system(dialog.system()))
            .add_system_set(
                SystemSet::on_enter(RoamState::Dialog).with_system(setup_dialog.system()),
            )
            .add_system_set(
                SystemSet::on_exit(RoamState::Dialog).with_system(destroy_dialog.system()),
            )
            .add_system_set(
                SystemSet::on_update(RoamState::Dialog)
                    .with_system(dialog.system())
                    .with_system(animate_spriteplex::<PandaManSpriteplex>.system())
                    .with_system(animate_spriteplex::<NPCSpriteplex>.system()),
            )
            .add_system_set(
                SystemSet::on_update(RoamState::Play)
                    .with_system(
                        animate_spriteplex::<PandaManSpriteplex>
                            .system()
                            .after("player_movement"),
                    )
                    .with_system(
                        animate_spriteplex::<NPCSpriteplex>
                            .system()
                            .after("npc_movement"),
                    )
                    .with_system(player_movement.system().label("player_movement"))
                    .with_system(
                        camera_movement
                            .system()
                            .label("camera")
                            .after("player_movement"),
                    )
                    .with_system(tile_interpolation.system().after("menu"))
                    .with_system(npc_movement.system().after("player_movement"))
                    .with_system(npc_interactions.system().after("npc_movement")),
            )
            .add_state(RoamState::Menu);
    }
}
