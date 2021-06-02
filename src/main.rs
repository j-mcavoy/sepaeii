use bevy::prelude::*;

mod roam;

fn main() {
    App::build().add_plugin(roam::plugin::RoamPlugin).run();
}
