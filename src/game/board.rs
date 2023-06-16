#[derive(Clone, Copy, PartialEq)]
pub enum BlockType {
    Empty = -2,
    Outside = -1,
    Red = 0,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
    Gray,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TSpinType {
    None,
    Normal,
    Mini,
}

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

    pub fn get(&self, x: i32, y: i32) -> BlockType {
        if x < 0 || y < 0 || x >= self.width as i32 {
            return BlockType::Outside;
        }

        if y >= self.rows.len() as i32 {
            return BlockType::Empty;
        }

        self.rows[y as usize].cells[x as usize]
    }

    pub fn set(&mut self, x: i32, y: i32, block: BlockType) {
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

    pub fn process_lines(&mut self) -> i32 {
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

    pub fn grayize(&mut self) {
        for y in 0..self.rows.len() as i32 {
            for x in 0..self.width as i32 {
                if self.rows[y as usize].cells[x as usize] != BlockType::Empty {
                    self.rows[y as usize].cells[x as usize] = BlockType::Gray;
                }
            }
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

struct Row {
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
