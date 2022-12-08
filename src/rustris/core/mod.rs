mod bag;
mod block_type;
mod board;
mod input_type;
mod input;
mod piece_type;
mod piece;

use bag::*;
pub use block_type::*;
use board::*;
pub use input_type::*;
pub use input::*;
use piece_type::*;
use piece::*;

pub struct Core {
    pub board: Board,
    pub bag: Bag,
    pub current_piece: Option<Piece>,
    pub input: Input,

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
                self.softdrop_delay = self.sdf;
            }

            if self.input.holded(InputType::SoftDrop) {
                self.softdrop_delay -= dt;

                while self.softdrop_delay <= 0.0f32 && piece.shift(&self.board, 0, -1) {
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
                }

                piece.place(&mut self.board);

                self.current_piece = None;
            }
        }

        self.input.update();
    }
}
