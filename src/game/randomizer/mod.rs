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
pub trait RandomizerImpl {
    fn pop(&mut self) -> PieceType;
}

#[enum_dispatch(RandomizerImpl)]
pub enum Randomizer {
    SingleBag,
    DoubleBag,
    Plain,
    GrandMaster3,
}

#[derive(Clone, Copy)]
pub enum RandomizerType {
    SingleBag,
    DoubleBag,
    Plain,
    GrandMaster3,
}

impl RandomizerType {
    pub fn to_struct(&self, seed: u64) -> Randomizer {
        match self {
            RandomizerType::SingleBag => SingleBag::new(seed),
            RandomizerType::DoubleBag => DoubleBag::new(seed),
            RandomizerType::Plain => Plain::new(seed),
            RandomizerType::GrandMaster3 => GrandMaster3::new(seed),
        }
    }
}
