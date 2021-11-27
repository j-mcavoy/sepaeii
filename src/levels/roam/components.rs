pub use crate::common::components::*;
use bevy::prelude::*;
use sepaeii_macros::SpriteplexM;

pub const SPAWN_POINT: (f32, f32) = (24. * 32., -8. * 32.);

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
    pub converstations: Vec<String>,
}
pub type Dialog = Vec<String>;

#[derive(Bundle)]
pub struct DialogUiBundle {
    dialog: Dialog,
    #[bundle]
    ui_camera_bundle: UiCameraBundle,
    text_bundle: TextBundle,
}
impl From<(Dialog, Transform, Handle<Font>)> for DialogUiBundle {
    fn from((dialog, transform, font): (Dialog, Transform, Handle<Font>)) -> Self {
        Self {
            dialog,
            ui_camera_bundle: UiCameraBundle {
                transform,
                ..Default::default()
            },
            text_bundle: TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(5.0),
                        right: Val::Px(15.0),
                        ..Default::default()
                    },
                    max_size: Size {
                        width: Val::Px(30.),
                        height: Val::Undefined,
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    "",
                    TextStyle {
                        font,
                        font_size: 30.0,
                        color: Color::rgb(1., 1., 1.),
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    },
                ),
                ..Default::default()
            },
        }
    }
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
