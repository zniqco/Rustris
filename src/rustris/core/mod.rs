mod bag;
mod block_type;
mod board;
mod input_type;
mod input;
mod piece_type;
mod piece;
mod tspin_type;

use bag::*;
pub use block_type::*;
use board::*;
pub use input_type::*;
pub use input::*;
use piece_type::*;
use piece::*;
use tspin_type::*;

pub struct Core {
    pub board: Board,
    pub bag: Bag,
    pub current_piece: Option<Piece>,
    pub input: Input,
    pub score: i32,
    pub level: i32,
    pub lines: i32,

    das: f32,
    arr: f32,
    sdf: f32,

    move_direction: i32,
    move_delay: f32,
    softdrop_delay: f32,
}

impl Core {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            bag: Bag::new(),
            current_piece: None,
            input: Input::new(),
            score: 0,
            level: 1,
            lines: 0,

            das: 10.0f32 / 60.0f32,
            arr: 1.0f32 / 60.0f32,
            sdf: 1.0f32 / 60.0f32,

            move_direction: 0,
            move_delay: 0.0f32,
            softdrop_delay: 0.0f32,
        }
    }

    pub fn init(&mut self) {
    }

    pub fn update(&mut self, dt: f32) {
        if let None = &mut self.current_piece {
            self.current_piece = Some(Piece::new(self.bag.pop()));
        }

        if let Some(piece) = &mut self.current_piece {
            // Move
            let direction = if self.input.pressed(InputType::MoveLeft) { -1 } else { 0 } +
                if self.input.pressed(InputType::MoveRight) { 1 } else { 0 };

            if direction != 0 && self.move_direction != direction {
                self.move_direction = direction;
                self.move_delay = self.das;

                piece.shift(&self.board, self.move_direction, 0);
            }
            
            if (self.input.released(InputType::MoveLeft) && self.move_direction == -1) ||
                (self.input.released(InputType::MoveRight) && self.move_direction == 1) {
                self.move_direction = 0;
            }

            if self.move_direction != 0 {
                self.move_delay -= dt;

                while self.move_delay <= 0.0f32 && piece.shift(&self.board, self.move_direction, 0) {
                    self.move_delay += self.arr;
                }
            }

            // Soft drop
            if self.input.pressed(InputType::SoftDrop) {
                piece.shift(&self.board, 0, -1);

                self.score += 1;
                self.softdrop_delay = self.sdf;
            }

            if self.input.holded(InputType::SoftDrop) {
                self.softdrop_delay -= dt;

                while self.softdrop_delay <= 0.0f32 && piece.shift(&self.board, 0, -1) {
                    self.score += 1;
                    self.softdrop_delay += self.sdf;
                }
            }

            // Rotate
            if self.input.pressed(InputType::RotateCW) {
                piece.rotate(&self.board, true);
            }

            if self.input.pressed(InputType::RotateCCW) {
                piece.rotate(&self.board, false);
            }

            if self.input.pressed(InputType::Flip) {
                piece.flip(&self.board);
            }

            // Hard drop
            if self.input.pressed(InputType::HardDrop) {
                while piece.shift(&self.board, 0, -1) {
                    self.score += 1;
                }

                piece.place(&mut self.board);

                self.process_lines();

                self.current_piece = None;
                self.move_delay = 0.0f32;
            }
        }

        self.input.update();
    }

    fn process_lines(&mut self) -> i32 {
        let tspin_state = match &self.current_piece {
            Some(piece) => piece.tspin_state,
            None => TSpinType::None,
        };

        let mut cleared = 0;
        let mut all_clear = true;
    
        for j in 0..self.board.get_height() {
            let mut is_cleared = true;
            let mut is_blank_line = true;
    
            for i in 0..self.board.get_width() {
                if self.board.get_block(i, j) == BlockType::Empty {
                    is_cleared = false;
                } else {
                    is_blank_line = false;
                }
            }
    
            if is_cleared {
                cleared += 1;
            } else {
                if !is_blank_line {
                    all_clear = false;
                }
    
                if cleared > 0 {
                    for i in 0..self.board.get_width() {
                        self.board.set_block(i, j - cleared, self.board.get_block(i, j));
                    }
    
                    if j >= self.board.get_height() - cleared {
                        for i in 0..self.board.get_width() {
                            self.board.set_block(i, j, BlockType::Empty);
                        }
                    }
                }
            }
        }

        let points = match tspin_state {
            TSpinType::None => match cleared {
                1 => match all_clear {
                    true => 800,
                    false => 100,
                },
                2 => match all_clear {
                    true => 1200,
                    false => 300,
                },
                3 => match all_clear {
                    true => 1800,
                    false => 500,
                },
                4 => match all_clear {
                    true => 2000,
                    false => 800,
                },
                _ => 0,
            },
            TSpinType::Normal => match cleared {
                1 => 400,
                2 => 800,
                3 => 1200,
                4 => 1600,
                _ => 0,
            },
            TSpinType::Mini => match cleared {
                0 => 100,
                1 => 200,
                2 => 400,
                _ => 0,
            },
        } * self.level;

        self.score += points;

        cleared
    }
}
