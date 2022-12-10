use super::*;

pub struct Board {
    rows: Vec<Row>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            rows: Vec::new(),
            width,
            height,
        }
    }

    pub fn get_block(&self, x: i32, y: i32) -> BlockType {
        if x < 0 || y < 0 || x >= self.width as i32 {
            return BlockType::Outside;
        }

        if y >= self.rows.len() as i32 {
            return BlockType::Empty;
        }

        self.rows[y as usize].cells[x as usize]
    }

    pub fn set_block(&mut self, x: i32, y: i32, block: BlockType) {
        if x < 0 || y < 0 || x >= self.width as i32 || block == BlockType::Outside {
            return;
        }

        if block != BlockType::Empty {
            while y >= self.rows.len() as i32 {
                self.rows.push(Row::new(self.width));
            }
        }

        if y < self.rows.len() as i32 {
            self.rows[y as usize].cells[x as usize] = block;
        }

        self.trim();
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn process_lines(&mut self) -> usize {
        let mut lines = 0;

        for i in (0..self.rows.len()).rev() {
            if self.rows[i].is_full() {
                self.rows.remove(i);
                lines += 1;
            }
        }

        self.trim();

        lines
    }

    pub fn is_cleared(&self) -> bool {
        match self.rows.len() {
            0 => true,
            _ => false,
        }
    }

    fn trim(&mut self) {
        for i in (0..self.rows.len()).rev() {
            if self.rows[i].is_empty() {
                self.rows.remove(i);
            } else {
                break;
            }
        }
    }
}
