mod common;
mod srs;
mod srs_plus;

pub use common::*;
pub use srs::*;
pub use srs_plus::*;

use enum_dispatch::enum_dispatch;
use super::*;

pub struct PieceData {
    pub shape: [&'static [&'static [i8]]; 4],
    pub block: BlockType,
    pub spawn_offset: (i32, i32),
    pub preview_offset: (f32, f32),
}

pub type Kick = [&'static [(i32, i32)]; 4];

pub struct KickData {
    pub cw: Kick,
    pub ccw: Kick,
    pub flip: Kick,
}

#[enum_dispatch]
pub trait Rotation {
    fn blocks(&self, piece: PieceType) -> &'static PieceData;
    fn kicks(&self, piece: PieceType) -> &'static KickData;
}

#[enum_dispatch(Rotation)]
pub enum RotationType {
    SRS,
    SRSPlus,
}
