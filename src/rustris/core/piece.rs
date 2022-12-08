use super::*;

pub struct Piece {
    piece_type: PieceType,
    size: (i32, i32),
    blocks: [[BlockType; 4]; 4],
    kicks: [[(i32, i32); 5]; 8],
    rotate_state: i32,

    pub x: i32,
    pub y: i32,
}

impl Piece {
    pub fn new(piece_type: PieceType) -> Self {
        let blocks = match piece_type {
            PieceType::Z => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Red, BlockType::Red, BlockType::Empty],
                [BlockType::Red, BlockType::Red, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::S => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Green, BlockType::Green, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Green, BlockType::Green, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::L => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Orange, BlockType::Orange, BlockType::Orange, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Orange, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::J => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Blue, BlockType::Blue, BlockType::Blue, BlockType::Empty],
                [BlockType::Blue, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::I => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Cyan, BlockType::Cyan, BlockType::Cyan, BlockType::Cyan],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::O => [
                [BlockType::Empty, BlockType::Yellow, BlockType::Yellow, BlockType::Empty],
                [BlockType::Empty, BlockType::Yellow, BlockType::Yellow, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::T => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Purple, BlockType::Purple, BlockType::Purple, BlockType::Empty],
                [BlockType::Empty, BlockType::Purple, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            _ => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ]
        };

        let size = match piece_type {
            PieceType::Z => (3, 3),
            PieceType::S => (3, 3),
            PieceType::L => (3, 3),
            PieceType::J => (3, 3),
            PieceType::I => (4, 4),
            PieceType::O => (4, 2),
            PieceType::T => (3, 3),
            _ => (0, 0),
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
            piece_type,
            blocks,
            size,
            kicks,
            x: 3,
            y: 21 - size.1,
            rotate_state: 0,
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

        for y in 0..self.size.0 as usize {
            for x in 0..self.size.1 as usize {
                if clockwise {
                    self.blocks[self.size.0 as usize - 1 - x][y] = copied[y][x];
                } else {
                    self.blocks[x][self.size.1 as usize - 1 - y] = copied[y][x];
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

        for y in 0..self.size.0 as usize {
            for x in 0..self.size.1 as usize {
                self.blocks[self.size.0 as usize - 1 - y][self.size.1 as usize - 1 - x] = copied[y][x];
            }
        }

        let next_rotate_state = (self.rotate_state + 2) % 4;

        if self.test(board) {
            self.rotate_state = next_rotate_state;

            return true;
        }

        self.blocks = copied;
    
        false
    }

    fn test(&self, board: &Board) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                let block = self.get_block(x, y);

                if block != BlockType::Empty {
                    let board_block = board.get_block(self.x + x, self.y + y);

                    if board_block != BlockType::Empty {
                        return false;
                    }
                }
            }
        }

        true
    }
}
