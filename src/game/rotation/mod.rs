mod common;
mod srs;
mod srs_plus;

use common::*;
use srs::*;
use srs_plus::*;

use enum_dispatch::enum_dispatch;
use super::*;

type Shape = [&'static [&'static [i8]]; 4];
type KickEntry = [&'static [(i32, i32)]; 4];

pub struct Kick {
    pub cw: KickEntry,
    pub ccw: KickEntry,
    pub flip: KickEntry,
}

#[derive(Clone, Copy)]
pub struct PieceData {
    pub shape: &'static Shape,
    pub kick: &'static Kick,
    pub block: BlockType,
    pub spawn_offset: (i32, i32),
    pub preview_offset: (f32, f32),
}

#[enum_dispatch]
pub trait RotationImpl {
    fn piece(&self, piece: PieceType) -> PieceData;
}

#[enum_dispatch(RotationImpl)]
pub enum Rotation {
    SRS,
    SRSPlus,
}

#[derive(Clone, Copy)]
pub enum RotationType {
    SRS,
    SRSPlus,
}

impl RotationType {
    pub fn to_struct(&self) -> Rotation {
        match self {
            RotationType::SRS => SRS::new(),
            RotationType::SRSPlus => SRSPlus::new(),
        }
    }
}
