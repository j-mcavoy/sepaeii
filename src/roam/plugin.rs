use bevy::{app::CoreStage::PreUpdate, prelude::*};

use super::systems::{animate_walkable::*, player_movement::*, setup::*, tile_interpolation::*};

use super::states::roam_state::RoamState;

pub struct RoamPlugin;

impl Plugin for RoamPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(bevy_tiled_prototype::TiledMapPlugin)
            .add_system(bevy::input::system::exit_on_esc_system.system())
            .add_state(RoamState::Play)
            .add_startup_system(setup.system())
            .add_system(player_movement.system())
            .add_system(animate_walkable.system())
            .add_system_to_stage(PreUpdate, tile_interpolation.system());
    }
}
