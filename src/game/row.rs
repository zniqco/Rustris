use super::*;

pub struct Row {
    pub cells: Vec<BlockType>,
}

impl Row {
    pub fn new(width: usize) -> Self {
        Self {
            cells: vec![BlockType::Empty; width],
        }
    }

    pub fn is_full(&self) -> bool {
        for i in 0..self.cells.len() {
            if self.cells[i] == BlockType::Empty {
                return false;
            }
        }

        true
    }

    pub fn is_empty(&self) -> bool {
        for i in 0..self.cells.len() {
            if self.cells[i] != BlockType::Empty {
                return false;
            }
        }

        true
    }
}