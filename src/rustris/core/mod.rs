mod bag;
mod block_type;
mod board;
mod config;
mod input_type;
mod input;
mod level_data;
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
pub use level_data::*;
pub use piece_type::*;
use piece::*;
use row::*;
use tspin_type::*;

pub struct Core {
    pub config: Config,
    pub board: Board,
    pub bag: Bag,
    pub current_piece: Option<Piece>,
    pub input: Input,
    pub score: i32,
    pub level: i32,
    pub lines: i32,
    pub hold_piece: Option<PieceType>,
    hold_enabled: bool,
    move_direction: i32,
    move_delay: f32,
    softdrop_delay: f32,
    gravity_delta: f32,
    lock_enabled: bool,
    pub lock_delta: f32,
    pub lock_force_delta: f32,
    level_data: LevelData,
    lines_offset: i32,
    game_over: bool,
}

impl Core {
    pub fn new(config: Config) -> Self {
        let width = config.width;
        let height = config.height;
        let first_level = config.levels[0];

        Self {
            config: config,
            board: Board::new(width, height),
            bag: Bag::new(None),
            current_piece: None,
            input: Input::new(),
            score: 0,
            level: 1,
            lines: 0,
            hold_piece: None,
            hold_enabled: true,
            move_direction: 0,
            move_delay: 0.0,
            softdrop_delay: 0.0,
            gravity_delta: 0.0,
            lock_enabled: false,
            lock_delta: 0.0,
            lock_force_delta: 0.0,
            level_data: first_level,
            lines_offset: 0,
            game_over: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        'inner: loop {
            if !self.game_over {
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
                    let mut new_piece = Piece::new(self.bag.pop(), self.board.width(), self.board.height());

                    if new_piece.test(&self.board, 0, 0) {
                        self.current_piece = Some(new_piece);
                    } else {
                        new_piece.place(&mut self.board);

                        for y in 0..self.board.row_count() as i32 {
                            for x in 0..self.board.width() as i32 {
                                if self.board.get_block(x, y) != BlockType::Empty {
                                    self.board.set_block(x, y, BlockType::Gray);
                                }
                            }
                        }

                        self.game_over = true;

                        break 'inner;
                    }
                }
            }

            if let Some(piece) = &mut self.current_piece {
                // Move
                let direction = if self.input.pressed(InputType::MoveLeft) { -1 } else { 0 } +
                    if self.input.pressed(InputType::MoveRight) { 1 } else { 0 };

                if direction != 0 && self.move_direction != direction {
                    self.move_direction = direction;
                    self.move_delay = self.config.das;

                    if piece.shift(&self.board, self.move_direction, 0) {
                        self.lock_delta = 0.0;
                    }
                }
                
                if (self.input.released(InputType::MoveLeft) && self.move_direction == -1) ||
                    (self.input.released(InputType::MoveRight) && self.move_direction == 1) {
                    self.move_direction = 0;
                }

                if self.move_direction != 0 {
                    self.move_delay -= dt;

                    while self.move_delay <= 0.0 && piece.shift(&self.board, self.move_direction, 0) {
                        self.move_delay += self.config.arr;
                        self.lock_delta = 0.0;
                    }
                }

                // Gravity
                self.gravity_delta += self.level_data.gravity * 60.0 * dt;

                while self.gravity_delta >= 1.0 {
                    if piece.shift(&self.board, 0, -1) {
                        self.gravity_delta -= 1.0;
                    } else {
                        break;
                    }
                }
                
                if self.lock_enabled {
                    self.lock_force_delta += dt / self.level_data.lock_delay / 4.0;
                }

                if !piece.test(&self.board, 0, -1) {
                    self.lock_delta += dt / self.level_data.lock_delay;

                    if self.lock_delta >= 1.0 || self.lock_force_delta >= 1.0 {
                        self.place_piece();

                        break 'inner;
                    }

                    self.lock_enabled = true;
                    self.gravity_delta = 0.0;
                } else {
                    self.lock_delta = 0.0;
                }

                // Soft drop
                if self.input.pressed(InputType::SoftDrop) {
                    if piece.shift(&self.board, 0, -1) {
                        self.score += 1;
                        self.gravity_delta = 0.0;
                    }

                    self.softdrop_delay = self.config.sdf;
                }

                if self.input.holded(InputType::SoftDrop) {
                    self.softdrop_delay -= dt;

                    while self.softdrop_delay <= 0.0 {
                        if piece.shift(&self.board, 0, -1) {
                            self.score += 1;
                            self.softdrop_delay += self.config.sdf;
                            self.gravity_delta = 0.0;
                        } else {
                            self.softdrop_delay = 0.0;

                            break;
                        }
                    }
                }

                // Rotate
                if self.input.pressed(InputType::RotateCW) {
                    if piece.rotate(&self.board, true) {
                        self.lock_delta = 0.0;
                    }
                }

                if self.input.pressed(InputType::RotateCCW) {
                    if piece.rotate(&self.board, false) {
                        self.lock_delta = 0.0;
                    }
                }

                if self.input.pressed(InputType::Flip) {
                    if piece.flip(&self.board) {
                        self.lock_delta = 0.0;
                    }
                }

                // Hard drop
                if self.input.pressed(InputType::HardDrop) {
                    while piece.shift(&self.board, 0, -1) {
                        self.score += 1;
                    }

                    self.place_piece();

                    break 'inner;
                }
            }

            break;
        }

        self.input.update();
    }

    fn place_piece(&mut self) {
        if let Some(piece) = &mut self.current_piece {
            piece.place(&mut self.board);
    
            let lines = self.board.process_lines();
            let cleared = self.board.is_cleared();
            let points = Self::get_points(self.level, lines, cleared, piece.tspin_state);

            self.score += points;
            self.lines += lines;
            
            if lines >= 1 || piece.tspin_state != TSpinType::None {
                println!("points={}, lines={}, tspin={:?}", points, lines, piece.tspin_state);
            }

            while self.level_data.lines > 0 && self.lines - self.lines_offset >= self.level_data.lines {
                self.lines_offset += self.level_data.lines;
                self.level += 1;
                self.level_data = self.config.levels[(self.level - 1) as usize];
            }

            self.current_piece = None;
            self.hold_enabled = true;
            self.move_delay = 0.0;
            self.gravity_delta = 0.0;
            self.lock_delta = 0.0;
            self.lock_force_delta = 0.0;
            self.lock_enabled = false;
        }
    }

    fn get_points(level: i32, lines: i32, cleared: bool, tspin_state: TSpinType) -> i32 {
        return match (lines, tspin_state, cleared) {
            (1, TSpinType::None, true) => 800,
            (1, TSpinType::None, false) => 100,
            (2, TSpinType::None, true) => 1200,
            (2, TSpinType::None, false) => 300,
            (3, TSpinType::None, true) => 1800,
            (3, TSpinType::None, false) => 500,
            (4, TSpinType::None, true) => 2000,
            (4, TSpinType::None, false) => 800,
            (0, TSpinType::Normal, _) => 400,
            (1, TSpinType::Normal, _) => 800,
            (2, TSpinType::Normal, _) => 1200,
            (3, TSpinType::Normal, _) => 1600,
            (0, TSpinType::Mini, _) => 100,
            (1, TSpinType::Mini, _) => 200,
            (2, TSpinType::Mini, _) => 400,
            _ => 0,
        } * level;
    }
}
