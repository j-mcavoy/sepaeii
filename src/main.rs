use bevy::prelude::*;

mod common;
mod levels;
use levels::*;

use levels::level01::plugin::Level01Plugin;
use levels::roam::plugin::RoamPlugin;

fn main() {
    App::build().add_plugin(Level01Plugin).run();
}
