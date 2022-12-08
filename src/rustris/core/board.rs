use super::*;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 24;

pub struct Board {
    blocks: [[BlockType; WIDTH]; HEIGHT],
    width: i32,
    height: i32,
}

impl Board {
    pub fn new() -> Self {
        Self {
            blocks: [[BlockType::Empty; WIDTH]; HEIGHT],
            width: WIDTH as i32,
            height: HEIGHT as i32,
        }
    }

    pub fn get_block(&self, x: i32, y: i32) -> BlockType {
        if y >= self.height {
            return BlockType::Empty;
        }

        if x < 0 || y < 0 || x >= self.width {
            return BlockType::Outside;
        }

        self.blocks[y as usize][x as usize]
    }

    pub fn set_block(&mut self, x: i32, y: i32, block: BlockType) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return;
        }

        self.blocks[y as usize][x as usize] = block;
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }
}
