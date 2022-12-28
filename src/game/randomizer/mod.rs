mod plain;
mod single_bag;
mod double_bag;
mod grand_master_3;

pub use plain::*;
pub use single_bag::*;
pub use double_bag::*;
pub use grand_master_3::*;

use enum_dispatch::enum_dispatch;
use super::*;

pub const PIECES: &[PieceType] = &[
    PieceType::Z, PieceType::S, PieceType::L, PieceType::J, PieceType::I, PieceType::O, PieceType::T
];

#[enum_dispatch]
pub trait Randomizer {
    fn pop(&mut self) -> PieceType;
}

#[enum_dispatch(Randomizer)]
pub enum RandomizerType {
    SingleBag,
    DoubleBag,
    Plain,
    GrandMaster3,
}
