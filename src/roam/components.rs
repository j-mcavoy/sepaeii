use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, Default)]
pub struct MainMenu;
#[derive(Debug, Clone, Default)]
pub struct Map;
#[derive(Debug, Clone, Default)]
pub struct PandaMan;

#[derive(Debug, Clone)]
pub struct AnimationTimer(pub Timer);

/// frames, flip_x?, flip_y?
#[derive(Debug, Clone)]
pub struct AnimationStrip(pub VecDeque<u32>, pub bool, pub bool);

impl From<Vec<u32>> for AnimationStrip {
    fn from(v: Vec<u32>) -> Self {
        Self(v.into(), false, false)
    }
}

#[derive(Debug, Clone)]
pub struct Walkable {
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

#[derive(Debug, Clone, Copy)]
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
