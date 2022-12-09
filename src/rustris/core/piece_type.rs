use super::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Z,
    S,
    L,
    J,
    I,
    O,
    T,
}

impl PieceType {
    pub fn get_block_type(&self) -> BlockType {
        match self {
            PieceType::Z => BlockType::Red,
            PieceType::S => BlockType::Green,
            PieceType::L => BlockType::Orange,
            PieceType::J => BlockType::Blue,
            PieceType::I => BlockType::Cyan,
            PieceType::O => BlockType::Yellow,
            PieceType::T => BlockType::Purple,
        }
    }
}