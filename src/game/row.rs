use super::*;

pub struct Row {
    width: usize,
    pub cells: Vec<BlockType>,
}

impl Row {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            cells: vec![BlockType::Empty; width],
        }
    }

    pub fn is_full(&self) -> bool {
        for i in 0..self.width {
            if self.cells[i] == BlockType::Empty {
                return false;
            }
        }

        true
    }

    pub fn is_empty(&self) -> bool {
        for i in 0..self.width {
            if self.cells[i] != BlockType::Empty {
                return false;
            }
        }

        true
    }
}