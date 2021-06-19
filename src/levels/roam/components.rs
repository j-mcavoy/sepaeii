use crate::common::components::*;
use bevy::prelude::*;
use sepaeii_macros::SpriteplexM;

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
#[derive(Debug, Clone, Copy, PartialEq, SpriteplexM)]
pub enum NPCState {
    StillUp,
    StillDown,
    StillLeft,
    StillRight,
}
impl Default for NPCState {
    fn default() -> Self {
        Self::StillDown
    }
}

#[derive(Debug, Clone, Default)]
pub struct Converstation {
    pub text: String,
    pub complete: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PandaMan;

#[derive(Debug, Copy, Clone, PartialEq, SpriteplexM)]
pub enum PandaManState {
    StillUp,
    StillDown,
    StillLeft,
    StillRight,
    WalkUp,
    WalkDown,
    WalkLeft,
    WalkRight,
}
impl Default for PandaManState {
    fn default() -> Self {
        Self::StillDown
    }
}
