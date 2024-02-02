use super::*;

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

#[derive(Clone)]
pub struct Piece {
    piece_type: PieceType,
    piece_data: PieceData,
    size: (i32, i32),
    position: (i32, i32),
    rotate_state: i32,
    tspin_state: TSpinState,
}

impl Piece {
    pub fn new(piece_type: PieceType, piece_data: PieceData, position: (i32, i32)) -> Self {
        let size = (piece_data.shape[0][0].len() as i32, piece_data.shape[0].len() as i32);

        Self {
            piece_type,
            piece_data,
            size,
            position,
            rotate_state: 0,
            tspin_state: TSpinState::None,
        }
    }

    pub fn width(&self) -> i32 {
        self.size.0
    }

    pub fn height(&self) -> i32 {
        self.size.1
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn tspin_state(&self) -> TSpinState {
        self.tspin_state
    }

    pub fn block_at(&self, x: i32, y: i32) -> BlockType {
        if self.piece_data.shape[self.rotate_state as usize][(self.size.1 - y - 1) as usize][x as usize] == 1 {
            self.piece_data.block
        } else {
            BlockType::Empty
        }
    }

    pub fn place(&mut self, board: &mut Board) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let block = self.block_at(x, y);

                if block != BlockType::Empty {
                    board.set(self.position.0 + x, self.position.1 + y, block);
                }
            }
        }
    }

    pub fn shift(&mut self, board: &Board, x: i32, y: i32) -> bool {
        if self.test(board, x, y) {
            self.position.0 += x;
            self.position.1 += y;
            self.tspin_state = TSpinState::None;

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
                if self.block_at(i, j) != BlockType::Empty && board.get(self.position.0 + i + x, self.position.1 + j + y) != BlockType::Empty {
                    return false;
                }
            }
        }

        true
    }

    fn rotate(&mut self, board: &Board, state_offset: i32) -> bool {
        let previous_rotate_state = self.rotate_state;
        let kick_table = match state_offset {
            1 => self.piece_data.kick.cw,
            3 => self.piece_data.kick.ccw,
            2 => self.piece_data.kick.flip,
            _ => panic!()
        }[self.rotate_state as usize];

        self.rotate_state = (self.rotate_state + state_offset) % 4;

        for kick in kick_table {
            if self.test(board, kick.0, kick.1) {
                self.position.0 += kick.0;
                self.position.1 += kick.1;    
                self.tspin_update(board);

                return true;
            }
        }

        self.rotate_state = previous_rotate_state;

        false
    }

    fn tspin_update(&mut self, board: &Board) {
        if self.piece_type() == PieceType::T {
            let corner_states = [
                if board.get(self.position.0, self.position.1 + 2) != BlockType::Empty { 1 } else { 0 },
                if board.get(self.position.0 + 2, self.position.1 + 2) != BlockType::Empty { 1 } else { 0 },
                if board.get(self.position.0 + 2, self.position.1) != BlockType::Empty { 1 } else { 0 },
                if board.get(self.position.0, self.position.1) != BlockType::Empty { 1 } else { 0 },
            ];

            if corner_states[0] + corner_states[1] + corner_states[2] + corner_states[3] >= 3 {
                let front_0 = ((0 + self.rotate_state) & 0x03) as usize;
                let front_1 = ((1 + self.rotate_state) & 0x03) as usize;

                if corner_states[front_0] + corner_states[front_1] == 2 {
                    self.tspin_state = TSpinState::Normal;
                } else {
                    self.tspin_state = TSpinState::Mini;
                }
            } else {
                self.tspin_state = TSpinState::None;
            }
        } else {
            self.tspin_state = TSpinState::None;
        }
    }
}
