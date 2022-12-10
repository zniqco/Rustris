mod bag;
mod block_type;
mod board;
mod config;
mod input_type;
mod input;
mod piece_type;
mod piece;
mod row;
mod tspin_type;

use bag::*;
pub use block_type::*;
use board::*;
pub use config::*;
pub use input_type::*;
pub use input::*;
use piece_type::*;
use piece::*;
use row::*;
use tspin_type::*;

pub struct Core {
    pub board: Board,
    pub bag: Bag,
    pub current_piece: Option<Piece>,
    pub input: Input,
    pub score: i32,
    pub level: i32,
    pub lines: i32,
    pub config: Config,
    hold_piece: Option<PieceType>,
    hold_enabled: bool,
    move_direction: i32,
    move_delay: f32,
    softdrop_delay: f32,
}

impl Core {
    pub fn new(config: Config) -> Self {
        Self {
            board: Board::new(10, 20),
            bag: Bag::new(None),
            current_piece: None,
            input: Input::new(),
            score: 0,
            level: 1,
            lines: 0,
            config,
            hold_piece: None,
            hold_enabled: true,
            move_direction: 0,
            move_delay: 0.0,
            softdrop_delay: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.input.pressed(InputType::Hold) && self.hold_enabled {
            if let Some(hold_piece) = self.hold_piece {
                self.bag.push_front(hold_piece);
            }

            if let Some(piece) = &mut self.current_piece {
                self.hold_piece = Some(piece.piece_type);
                self.current_piece = None;
            }

            self.hold_enabled = false;
        }

        if let None = &self.current_piece {
            self.current_piece = Some(Piece::new(self.bag.pop(), self.board.width(), self.board.height()));
        }

        if let Some(piece) = &mut self.current_piece {
            // Move
            let direction = if self.input.pressed(InputType::MoveLeft) { -1 } else { 0 } +
                if self.input.pressed(InputType::MoveRight) { 1 } else { 0 };

            if direction != 0 && self.move_direction != direction {
                self.move_direction = direction;
                self.move_delay = self.config.das;

                piece.shift(&self.board, self.move_direction, 0);
            }
            
            if (self.input.released(InputType::MoveLeft) && self.move_direction == -1) ||
                (self.input.released(InputType::MoveRight) && self.move_direction == 1) {
                self.move_direction = 0;
            }

            if self.move_direction != 0 {
                self.move_delay -= dt;

                while self.move_delay <= 0.0 && piece.shift(&self.board, self.move_direction, 0) {
                    self.move_delay += self.config.arr;
                }
            }

            // Soft drop
            if self.input.pressed(InputType::SoftDrop) {
                if piece.shift(&self.board, 0, -1) {
                    self.score += 1;
                }

                self.softdrop_delay = self.config.sdf;
            }

            if self.input.holded(InputType::SoftDrop) {
                self.softdrop_delay -= dt;

                while self.softdrop_delay <= 0.0 {
                    if piece.shift(&self.board, 0, -1) {
                        self.score += 1;
                        self.softdrop_delay += self.config.sdf;
                    } else {
                        self.softdrop_delay = 0.0;

                        break;
                    }
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
 
                let lines = self.board.process_lines();
                let cleared = self.board.is_cleared();
                let points = Self::get_points(self.level, lines, cleared, piece.tspin_state);

                self.score += points;

                if lines >= 1 || piece.tspin_state != TSpinType::None {
                    println!("points={}, lines={}, tspin={:?}", points, lines, piece.tspin_state);
                }

                self.current_piece = None;
                self.hold_enabled = true;
                self.move_delay = 0.0;
            }
        }

        self.input.update();
    }

    fn get_points(level: i32, lines: usize, cleared: bool, tspin_state: TSpinType) -> i32 {
        return match tspin_state {
            TSpinType::None => match lines {
                1 => match cleared {
                    true => 800,
                    false => 100,
                },
                2 => match cleared {
                    true => 1200,
                    false => 300,
                },
                3 => match cleared {
                    true => 1800,
                    false => 500,
                },
                4 => match cleared {
                    true => 2000,
                    false => 800,
                },
                _ => 0,
            },
            TSpinType::Normal => match lines {
                0 => 400,
                1 => 800,
                2 => 1200,
                3 => 1600,
                _ => 0,
            },
            TSpinType::Mini => match lines {
                0 => 100,
                1 => 200,
                2 => 400,
                _ => 0,
            },
        } * level;
    }
}
