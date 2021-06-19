use super::super::super::common::components::*;
use crate::common::components::*;
use bevy::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct MainMenu;
pub mod map_layers {
    pub const BACKGROUND: usize = 0;
    pub const GROUND1: usize = 1;
    pub const OBJECTS: usize = 2;
    pub const GROUND2: usize = 3;
    pub const OVERHEAD1: usize = 4;
    pub const OVERHEAD2: usize = 5;
    pub const OVERHEAD3: usize = 6;
    pub const OVERHEAD4: usize = 7;
}

#[derive(Debug, Clone, Default)]
pub struct Map;
#[derive(Debug, Clone, Default)]
pub struct NPC {
    //pub converstations: Vec<Converstation>,
//pub converstation_index: usize,
}
#[derive(Debug, Clone, Default)]
pub struct Converstation {
    pub text: String,
    pub complete: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PandaMan;
#[derive(Debug, Default, Clone)]
pub struct PandaManSprux {
    pub still_up: AnimationStrip,
    pub still_down: AnimationStrip,
    pub still_left: AnimationStrip,
    pub still_right: AnimationStrip,
    pub walk_up: AnimationStrip,
    pub walk_down: AnimationStrip,
    pub walk_left: AnimationStrip,
    pub walk_right: AnimationStrip,
    pub state: WalkableState,
}
impl PandaManSprux {
    pub fn reset_animation_strip(&mut self) {
        let strip = match self.state {
            WalkableState::StillUp => &mut self.still_up,
            WalkableState::StillDown => &mut self.still_down,
            WalkableState::StillLeft => &mut self.still_left,
            WalkableState::StillRight => &mut self.still_right,
            WalkableState::WalkUp => &mut self.walk_up,
            WalkableState::WalkDown => &mut self.walk_down,
            WalkableState::WalkLeft => &mut self.walk_left,
            WalkableState::WalkRight => &mut self.walk_right,
        };
        strip.reset();
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WalkableState {
    StillUp,
    StillDown,
    StillLeft,
    StillRight,
    WalkUp,
    WalkDown,
    WalkLeft,
    WalkRight,
}
impl Default for WalkableState {
    fn default() -> Self {
        Self::StillDown
    }
}
