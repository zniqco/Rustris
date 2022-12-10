use super::*;

#[derive(Clone)]
pub struct Piece {
    size: (usize, usize),
    blocks: [[BlockType; 4]; 4],
    kicks: [[(i32, i32); 5]; 8],
    rotate_state: i32,
    
    pub piece_type: PieceType,
    pub x: i32,
    pub y: i32,
    pub tspin_state: TSpinType,
}

impl Piece {
    pub fn new(piece_type: PieceType, board_width: usize, board_height: usize) -> Self {
        let block_type = piece_type.get_block_type();

        let blocks = match piece_type {
            PieceType::Z => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, block_type, block_type, BlockType::Empty],
                [block_type, block_type, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::S => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [block_type, block_type, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, block_type, block_type, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::L => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [block_type, block_type, block_type, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, block_type, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::J => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [block_type, block_type, block_type, BlockType::Empty],
                [block_type, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::I => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [block_type, block_type, block_type, block_type],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::O => [
                [BlockType::Empty, block_type, block_type, BlockType::Empty],
                [BlockType::Empty, block_type, block_type, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::T => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [block_type, block_type, block_type, BlockType::Empty],
                [BlockType::Empty, block_type, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
        };

        let size = match piece_type {
            PieceType::Z => (3, 3),
            PieceType::S => (3, 3),
            PieceType::L => (3, 3),
            PieceType::J => (3, 3),
            PieceType::I => (4, 4),
            PieceType::O => (4, 2),
            PieceType::T => (3, 3),
        };

        let kicks = match piece_type {
            PieceType::Z | PieceType::S | PieceType::L | PieceType::J | PieceType::T => [
                [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
            ],
            PieceType::I => [
                [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
            ],
            _ => [[(0, 0); 5]; 8],
        };

        Self {
            blocks,
            size,
            kicks,
            rotate_state: 0,
            piece_type,
            x: board_width as i32 / 2 - 2,
            y: board_height as i32 + 1 - size.1 as i32,
            tspin_state: TSpinType::None,
        }
    }

    pub fn get_block(&self, x: i32, y: i32) -> BlockType {
        self.blocks[y as usize][x as usize]
    }

    pub fn place(&mut self, board: &mut Board) {
        for y in 0..4 {
            for x in 0..4 {
                let block = self.get_block(x, y);

                if block != BlockType::Empty {
                    board.set_block(self.x + x, self.y + y, block);
                }
            }
        }
    }

    pub fn shift(&mut self, board: &Board, x: i32, y: i32) -> bool {
        self.x += x;
        self.y += y;

        if !self.test(board) {
            self.x -= x;
            self.y -= y;

            false
        } else {
            self.tspin_state = TSpinType::None;

            true
        }
    }

    pub fn rotate(&mut self, board: &Board, clockwise: bool) -> bool {
        if self.size.0 != self.size.1 {
            return true;
        }

        let previous_x = self.x;
        let previous_y = self.y;
        let copied = self.blocks.clone();

        for y in 0..self.size.0 {
            for x in 0..self.size.1 {
                if clockwise {
                    self.blocks[self.size.0 - 1 - x][y] = copied[y][x];
                } else {
                    self.blocks[x][self.size.1 - 1 - y] = copied[y][x];
                }
            }
        }

        let next_rotate_state = (self.rotate_state + if clockwise { 1 } else { 3 }) % 4;
        let kick_table = self.kicks[if clockwise { self.rotate_state * 2 } else { next_rotate_state * 2 + 1 } as usize];

        for i in 0..5 {
            self.x = previous_x + kick_table[i].0;
            self.y = previous_y + kick_table[i].1;

            if self.test(board) {
                self.rotate_state = next_rotate_state;
                self.tspin_update(board);

                return true;
            }
        }

        self.x = previous_x;
        self.y = previous_y;
        self.blocks = copied;
    
        false
    }
    
    pub fn flip(&mut self, board: &Board) -> bool {
        if self.size.0 != self.size.1 {
            return true;
        }

        let copied = self.blocks.clone();

        for y in 0..self.size.0 {
            for x in 0..self.size.1 {
                self.blocks[self.size.0 - 1 - y][self.size.1 - 1 - x] = copied[y][x];
            }
        }

        let next_rotate_state = (self.rotate_state + 2) % 4;

        if self.test(board) {
            self.rotate_state = next_rotate_state;
            self.tspin_update(board);

            return true;
        }

        self.blocks = copied;
    
        false
    }

    fn test(&self, board: &Board) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if self.get_block(x, y) != BlockType::Empty && board.get_block(self.x + x, self.y + y) != BlockType::Empty {
                    return false;
                }
            }
        }

        true
    }

    fn tspin_update(&mut self, board: &Board) {
        if self.piece_type == PieceType::T {
            let corner_states = [
                if board.get_block(self.x, self.y + 2) != BlockType::Empty { 1 } else { 0 },
                if board.get_block(self.x + 2, self.y + 2) != BlockType::Empty { 1 } else { 0 },
                if board.get_block(self.x + 2, self.y) != BlockType::Empty { 1 } else { 0 },
                if board.get_block(self.x, self.y) != BlockType::Empty { 1 } else { 0 },
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
