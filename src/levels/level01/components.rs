use crate::common::components::*;
use bevy::prelude::*;

use sepaeii_macros::SpriteplexM;

#[derive(Debug, Clone, Default)]
pub struct MainMenu;
pub mod map_layers {
    pub const BACKGROUND: usize = 1;
    pub const OBJECTS: usize = 2;
}

#[derive(Debug, Clone, Default)]
pub struct Map;
#[derive(Debug, Clone, Default)]
pub struct PandaMan;
#[derive(Debug, Clone, Default)]
pub struct NPC {
    //pub converstations: Vec<Converstation>,
//pub converstation_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PowerUp {
    Star,
    FirePower,
}

// TODO: write macro to derive stated sprite logic

//#[derive(Debug, Default, Clone)]
pub struct Mario;
//{
//    pub is_big: bool,
//    pub powerup: Option<PowerUp>,
//    pub state: CharacterState,
//    pub sm_mario_jump: AnimationStrip,
//    pub sm_mario_crouch: AnimationStrip,
//    pub sm_mario_still_left: AnimationStrip,
//    pub sm_mario_still_right: AnimationStrip,
//    pub sm_mario_walk_left: AnimationStrip,
//    pub sm_mario_walk_right: AnimationStrip,
//}
//impl Mario {
//    pub fn get_animation_strip(&self) -> AnimationStrip {
//        match self.state {}
//    }
//    pub fn reset_animation_strip(&mut self) {
//        let strip = match self.state {
//            CharacterState::StillLeft => &mut self.still_left,
//            CharacterState::StillRight => &mut self.still_right,
//            CharacterState::WalkLeft => &mut self.walk_left,
//            CharacterState::WalkRight => &mut self.walk_right,
//            CharacterState::Jump => &mut self.walk_right,
//        };
//        strip.reset();
//    }
//}

#[derive(Debug, Clone, Copy, PartialEq, SpriteplexM)]
pub enum CharacterState {
    Jump,
    StillLeft,
    StillRight,
    WalkLeft,
    WalkRight,
    Crouch,
}
impl Default for CharacterState {
    fn default() -> Self {
        Self::StillRight
    }
}

#[cfg(test)]
mod test {
    use super::AnimationStrip;
    use super::SpriteplexM;
    use crate::common::components::Spriteplex;

    #[derive(Debug, Copy, Clone, SpriteplexM)]
    enum PandaManState {
        WalkUp,
        WalkDown,
        WalkLeft,
        WalkRight,
        StillUp,
        StillDown,
        StillLeft,
        StillRight,
    }
    impl Default for PandaManState {
        fn default() -> Self {
            Self::StillUp
        }
    }

    #[test]
    fn expansion() {
        let pandaman = PandaManState::WalkUp;
        let pandaman_sp = PandaManSpriteplex::default();
    }
}
