use bevy::prelude::*;

mod levels;

fn main() {
    App::build()
        .add_plugin(levels::roam::plugins::RoamPlugin)
        .run();
}
