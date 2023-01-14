use super::*;

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
pub enum PieceType {
    Z,
    L,
    O,
    S,
    I,
    J,
    T,
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


#[derive(Clone)]
pub struct Piece {
    piece_data: &'static PieceData,
    kick_data: &'static KickData,
    size: (i32, i32),
    rotate_state: i32,
    x: i32,
    y: i32,
    piece_type: PieceType,
    block_type: BlockType,
    tspin_state: TSpinType,
}

impl Piece {
    pub fn new(rotation: &RotationType, piece: PieceType, board_width: usize, board_height: usize) -> Self {
        let piece_data = rotation.blocks(piece);
        let kick_data = rotation.kicks(piece);
        let size = (piece_data.shape[0][0].len() as i32, piece_data.shape[0].len() as i32);

        Self {
            piece_data,
            kick_data,
            size,
            rotate_state: 0,
            x: board_width as i32 / 2 + piece_data.spawn_offset.0,
            y: board_height as i32 + piece_data.spawn_offset.1,
            piece_type: piece,
            block_type: piece_data.block,
            tspin_state: TSpinType::None,
        }
    }

    pub fn width(&self) -> i32 {
        self.size.0
    }

    pub fn height(&self) -> i32 {
        self.size.1
    }

    pub fn rotate_state(&self) -> i32 {
        self.rotate_state
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn block_type(&self) -> BlockType {
        self.block_type
    }

    pub fn tspin_state(&self) -> TSpinType {
        self.tspin_state
    }

    pub fn block_at(&self, x: i32, y: i32) -> BlockType {
        if self.piece_data.shape[self.rotate_state as usize][(self.size.1 - y - 1) as usize][x as usize] == 1 {
            self.block_type
        } else {
            BlockType::Empty
        }
    }

    pub fn place(&mut self, board: &mut Board) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let block = self.block_at(x, y);

                if block != BlockType::Empty {
                    board.set(self.x + x, self.y + y, block);
                }
            }
        }
    }

    pub fn shift(&mut self, board: &Board, x: i32, y: i32) -> bool {
        if self.test(board, x, y) {
            self.x += x;
            self.y += y;
            self.tspin_state = TSpinType::None;

            true
        } else {
            false
        }
    }

    pub fn cw(&mut self, board: &Board) -> bool {
        self.rotate(board, 1)
    }

    pub fn ccw(&mut self, board: &Board) -> bool {
        self.rotate(board, 3)
    }

    pub fn flip(&mut self, board: &Board) -> bool {
        self.rotate(board, 2)
    }

    pub fn test(&self, board: &Board, x: i32, y: i32) -> bool {
        for j in 0..self.size.1 {
            for i in 0..self.size.0 {
                if self.block_at(i, j) != BlockType::Empty && board.get(self.x + i + x, self.y + j + y) != BlockType::Empty {
                    return false;
                }
            }
        }

        true
    }

    fn rotate(&mut self, board: &Board, state_offset: i32) -> bool {
        let previous_rotate_state = self.rotate_state;
        let kick_table = match state_offset {
            1 => self.kick_data.cw,
            3 => self.kick_data.ccw,
            2 => self.kick_data.flip,
            _ => panic!()
        }[self.rotate_state as usize];

        self.rotate_state = (self.rotate_state + state_offset) % 4;

        for i in 0..kick_table.len() {
            if self.test(board, kick_table[i].0, kick_table[i].1) {
                self.x += kick_table[i].0;
                self.y += kick_table[i].1;    
                self.tspin_update(board);

                return true;
            }
        }

        self.rotate_state = previous_rotate_state;

        false
    }

    fn tspin_update(&mut self, board: &Board) {
        if self.piece_type == PieceType::T {
            let corner_states = [
                if board.get(self.x, self.y + 2) != BlockType::Empty { 1 } else { 0 },
                if board.get(self.x + 2, self.y + 2) != BlockType::Empty { 1 } else { 0 },
                if board.get(self.x + 2, self.y) != BlockType::Empty { 1 } else { 0 },
                if board.get(self.x, self.y) != BlockType::Empty { 1 } else { 0 },
            ];

            if corner_states[0] + corner_states[1] + corner_states[2] + corner_states[3] >= 3 {
                let front_0 = ((0 + self.rotate_state) & 0x03) as usize;
                let front_1 = ((1 + self.rotate_state) & 0x03) as usize;

                if corner_states[front_0] + corner_states[front_1] == 2 {
                    self.tspin_state = TSpinType::Normal;
                } else {
                    self.tspin_state = TSpinType::Mini;
                }
            } else {
                self.tspin_state = TSpinType::None;
            }
        } else {
            self.tspin_state = TSpinType::None;
        }
    }
}
