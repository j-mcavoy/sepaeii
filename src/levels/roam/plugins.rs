use super::systems::*;
use bevy::{app::CoreStage::PreUpdate, prelude::*};
use bevy_tiled_prototype::{MapReadyEvent, TiledMapCenter};
use std::collections::VecDeque;

pub struct RoamPlugin;

impl Plugin for RoamPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(bevy_tiled_prototype::TiledMapPlugin)
            .add_system(bevy::input::system::exit_on_esc_system.system())
            .add_startup_system(setup.system())
            .add_system(player_movement.system())
            .add_system(animate_walkable.system())
            .add_system_to_stage(PreUpdate, set_texture_filters_to_nearest.system());
    }
}
