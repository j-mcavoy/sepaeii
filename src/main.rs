use bevy::prelude::*;

mod common;
mod levels;
use levels::*;

use levels::roam::plugin::RoamPlugin;

fn main() {
    App::build().add_plugin(RoamPlugin).run();
}
