use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, Default)]
pub struct MainMenu;
pub mod map_layers {
    pub const BACKGROUND: f32 = 2.;
    pub const GROUND1: f32 = 4.;
    pub const OBJECTS: f32 = 5.;
    pub const GROUND2: f32 = 6.;
    pub const OVERHEAD1: f32 = 7.;
    pub const OVERHEAD2: f32 = 8.;
    pub const OVERHEAD3: f32 = 9.;
    pub const OVERHEAD4: f32 = 10.;
}
#[derive(Debug, Clone, Default)]
pub struct Map;
#[derive(Debug, Clone, Default)]
pub struct PandaMan;

#[derive(Debug, Clone)]
pub struct AnimationTimer(pub Timer);

/// frames, flip_x?, flip_y?
#[derive(Debug, Clone, Default)]
pub struct AnimationStrip {
    pub sequence: Vec<u32>,
    pub flip_x: bool,
    pub flip_y: bool,
    pub index: usize,
}
impl AnimationStrip {
    pub fn get_frame(&self) -> u32 {
        self.sequence[self.index]
    }
    pub fn next_frame(&mut self) {
        self.index += 1;
        self.index %= self.sequence.len();
    }
    pub fn reset(&mut self) {
        self.index = 0;
        println!("reset");
    }
}
impl<T: Into<Vec<u32>>> From<T> for AnimationStrip {
    fn from(t: T) -> Self {
        Self {
            sequence: t.into(),
            ..Default::default()
        }
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

impl Walkable {
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
