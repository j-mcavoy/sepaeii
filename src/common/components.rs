use bevy::{ecs::component::Component, prelude::*};

pub trait Spriteplex: Component {
    fn get_animation_strip(&self) -> AnimationStrip;
    fn next_frame(&mut self);
    fn reset_animation_strip(&mut self);
}

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

#[derive(Bundle)]
pub struct BoxColliderBundle {
    pub collider: BoxCollider,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
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
    assert_eq!(Vec2::new(1., 7.), c.sw());
}
impl Collider for BoxCollider {
    fn collides(&self, point: Vec2) -> bool {
        let nw = self.nw();
        let se = self.se();
        (nw.x <= point.x && nw.y <= point.y) || (se.x >= point.x && se.y >= point.y)
    }
}
