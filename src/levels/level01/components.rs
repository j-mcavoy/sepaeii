use crate::common::components::*;

use bevy::math::Vec3;
use sepaeii_macros::SpriteplexM;

#[derive(Debug, Clone, Default)]
pub struct MainMenu;
pub mod map_layers {
    pub const BACKGROUND: usize = 1;
    pub const OBJECTS: usize = 1;
}

#[derive(Debug, Clone, Default)]
pub struct Ground;

#[derive(Debug, Clone, Default)]
pub struct Brick {
    item: Option<Item>,
    disabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct StaticObject;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Item {
    PowerUp(PowerUp),
    Mushroom,
    Coin,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerUp {
    Star,
    FirePower,
}

#[derive(Debug, Default, Clone)]
pub struct Mario {
    pub is_big: bool,
    pub is_grounded: bool,
    pub is_crouching: bool,
    pub powerup: Option<PowerUp>,
    pub velocity: Vec3,
}
impl Mario {
    pub fn next_state(self) -> MarioState {
        
        let Mario {
            is_big: _,
            is_grounded: _,
            is_crouching: _,
            powerup: _,
            velocity: _,
        } = self;

        MarioState::SmallMarioWalkRight
    }
}

#[derive(Debug, Clone, Copy, PartialEq, SpriteplexM)]
pub enum MarioState {
    SmallMarioJump,
    SmallMarioStillLeft,
    SmallMarioStillRight,
    SmallMarioWalkLeft,
    SmallMarioWalkRight,
    SmallMarioCrouchRight,
    SmallMarioCrouchLeft,
    SmallStarJump,
    SmallStarStillLeft,
    SmallStarStillRight,
    SmallStarWalkLeft,
    SmallStarWalkRight,
    SmallStarCrouch,
    SmallFirePowerStillRight,
    SmallFirePowerWalkLeft,
    SmallFirePowerWalkRight,
    SmallFirePowerCrouchRight,
    SmallFirePowerCrouchLeft,
    BigMarioJump,
    BigMarioStillLeft,
    BigMarioStillRight,
    BigMarioWalkLeft,
    BigMarioWalkRight,
    BigMarioCrouchRight,
    BigMarioCrouchLeft,
    BigStarJump,
    BigStarStillLeft,
    BigStarStillRight,
    BigStarWalkLeft,
    BigStarWalkRight,
    BigStarCrouchRight,
    BigStarCrouchLeft,
    BigFirePowerJump,
    BigFirePowerStillLeft,
    BigFirePowerStillRight,
    BigFirePowerWalkLeft,
    BigFirePowerWalkRight,
    BigFirePowerCrouchRight,
    BigFirePowerCrouchLeft,
}
impl Default for MarioState {
    fn default() -> Self {
        Self::SmallMarioStillRight
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
