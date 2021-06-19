use bevy::prelude::*;

mod common;
mod levels;
use levels::*;

fn main() {
    App::build()
        //.add_plugin(roam::plugin::RoamPlugin)
        .add_plugin(level01::plugin::Level01Plugin)
        .run();
}
