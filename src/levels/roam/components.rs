use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, Default)]
pub struct PandaMan;

#[derive(Debug, Clone)]
pub struct AnimationTimer(pub Timer);

#[derive(Debug, Clone)]
pub struct Walkable {
    pub still_up: VecDeque<u32>,
    pub still_down: VecDeque<u32>,
    pub still_left: VecDeque<u32>,
    pub still_right: VecDeque<u32>,
    pub walk_up: VecDeque<u32>,
    pub walk_down: VecDeque<u32>,
    pub walk_left: VecDeque<u32>,
    pub walk_right: VecDeque<u32>,
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
