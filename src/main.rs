use bevy::prelude::*;

mod level01;
mod roam;

fn main() {
    App::build()
        //.add_plugin(roam::plugin::RoamPlugin)
        .add_plugin(level01::plugin::Level01Plugin)
        .run();
}
