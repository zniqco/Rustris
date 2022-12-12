use super::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Z,
    L,
    O,
    S,
    I,
    J,
    T,
}

impl PieceType {
    pub fn get_block_type(&self) -> BlockType {
        match self {
            PieceType::Z => BlockType::Red,
            PieceType::L => BlockType::Orange,
            PieceType::O => BlockType::Yellow,
            PieceType::S => BlockType::Green,
            PieceType::I => BlockType::Cyan,
            PieceType::J => BlockType::Blue,
            PieceType::T => BlockType::Purple,
        }
    }
}