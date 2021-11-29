use std::collections::VecDeque;

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
pub type Dialog = VecDeque<String>;
pub struct DialogUi;

#[derive(Bundle)]
pub struct DialogUiBundle {
    pub dialog: Dialog,
    pub dialog_ui: DialogUi,
    //#[bundle]
    //pub ui_camera_bundle: UiCameraBundle,
    #[bundle]
    pub text_bundle: TextBundle,
}
impl From<(Dialog, Handle<Font>)> for DialogUiBundle {
    fn from((mut dialog, font): (Dialog, Handle<Font>)) -> Self {
        let text = dialog.pop_front().expect("empty dialog vec").clone();
        Self {
            dialog,
            dialog_ui: DialogUi,
            text_bundle: TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        bottom: Val::Px(5.0),
                        right: Val::Px(15.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    text,
                    TextStyle {
                        font,
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..Default::default()
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
