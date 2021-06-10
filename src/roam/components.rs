
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
pub struct PandaMan;

trait Collider {
    fn collides(&self, point: Vec2) -> bool;
    //fn collides(&self, collider: T: Collides) -> bool;
}
pub enum ColliderError {
    InvalidGeometry,
}
#[derive(Debug, Clone, Default)]
pub struct BoxCollider {
    pub width: f32,
    pub height: f32,
    pub origin: Vec2,
}
impl BoxCollider {
    pub fn nw(&self) -> Vec2 {
        self.origin
    }
    pub fn ne(&self) -> Vec2 {
        self.origin + Vec2::new(self.width, 0.)
    }
    pub fn sw(&self) -> Vec2 {
        self.origin + Vec2::new(0., self.height)
    }
    pub fn se(&self) -> Vec2 {
        self.origin + Vec2::new(self.width, self.height)
    }
    pub fn corners(&self) -> Vec<Vec2> {
        vec![self.nw(), self.ne(), self.sw(), self.se()]
    }
}
#[test]
fn collider() {
    let c = BoxCollider {
        width: 10.,
        height: 5.,
        origin: (1., 2.).into(),
    };
    assert_eq!(Vec2::new(1., 2.), c.nw());
    assert_eq!(Vec2::new(11., 2.), c.ne());
    assert_eq!(Vec2::new(11., 7.), c.se());
    assert_eq!(Vec2::new(1.  7.), c.sw());
}
impl Collider for BoxCollider {
    fn collides(&self, point: Vec2) -> bool {
        let nw = self.nw();
        let se = self.se();
        (nw.x <= point.x && nw.y <= point.y) || (se.x >= point.x && se.y >= point.y)
    }
}

#[derive(Bundle)]
pub struct BoxColliderBundle {
    pub collider: BoxCollider,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

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
