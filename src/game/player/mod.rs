mod user;

pub use user::*;

use enum_dispatch::enum_dispatch;
use super::*;

#[enum_dispatch]
pub trait Player {
    fn update(&mut self, board: &Board, piece: &Option<Piece>, bag: &Vec<PieceType>, hold: Option<PieceType>);
    fn horizontal(&mut self) -> i32;
    fn soft_drop(&mut self) -> bool;
    fn hard_drop(&mut self) -> bool;
    fn cw(&mut self) -> bool;
    fn ccw(&mut self) -> bool;
    fn flip(&mut self) -> bool;
    fn hold(&mut self) -> bool;
}

#[enum_dispatch(Player)]
pub enum PlayerType {
    User,
}
