use std::collections::VecDeque;
use super::*;

const MINIMUM_NEXT_COUNT: usize = 7;

pub enum EventType {
    LockReset,
    Pointed { score: i32, lines: i32, combo: i32, b2b: bool, tspin: TSpinType },
}

pub struct Game {
    pub config: Config,
    pub board: Board,
    pub score: i32,
    pub level: i32,
    pub lines: i32,
    pub current_piece: Option<Piece>,
    pub bag: Vec<PieceType>,
    pub player: PlayerType,
    randomizer: RandomizerType,
    pub rotation: RotationType,
    pub hold_piece: Option<PieceType>,
    pub hold_enabled: bool,
    move_direction: i32,
    move_delay: f32,
    softdrop_delay: f32,
    softdroping: bool,
    gravity_delta: f32,
    lock_enabled: bool,
    pub lock_delta: f32,
    pub lock_force_delta: f32,
    level_data: LevelData,
    lines_offset: i32,
    combo: i32,
    b2b_combo: i32,
    game_over: bool,
}

impl Game {
    pub fn new(config: Config) -> Self {
        let width = config.width;
        let height = config.height;
        let seed = config.seed;
        let first_level = config.levels[0];
        let mut instance = Self {
            config: config,
            board: Board::new(width, height),
            score: 0,
            level: 1,
            lines: 0,
            current_piece: None,
            bag: Vec::new(),
            player: User::new(),
            randomizer: SingleBag::new(seed),
            rotation: SRS::new(),
            hold_piece: None,
            hold_enabled: true,
            move_direction: 0,
            move_delay: 0.0,
            softdrop_delay: 0.0,
            softdroping: false,
            gravity_delta: 0.0,
            lock_enabled: false,
            lock_delta: 0.0,
            lock_force_delta: 0.0,
            level_data: first_level,
            lines_offset: 0,
            combo: 0,
            b2b_combo: 0,
            game_over: false,
        };

        instance.fill_next();

        instance
    }

    pub fn update(&mut self, dt: f32) -> Vec<EventType> {
        let mut events = Vec::new();

        self.player.update(&self.board, &self.current_piece, &self.bag, self.hold_piece);

        'inner: loop {
            if !self.game_over {
                if self.player.hold() && self.hold_enabled {
                    if let Some(piece) = &mut self.current_piece {
                        if let Some(hold_piece) = self.hold_piece {
                            self.bag.insert(0, hold_piece);
                        }

                        self.hold_piece = Some(piece.piece_type());
                        self.current_piece = None;
                        self.hold_enabled = false;
                    }
                }

                if let None = &self.current_piece {
                    let mut new_piece = Piece::new(&self.rotation, self.bag[0], self.board.width(), self.board.height());

                    self.bag.remove(0);
                    self.fill_next();

                    if new_piece.test(&self.board, 0, 0) {
                        self.current_piece = Some(new_piece);
                    } else if new_piece.test(&self.board, 0, 1) {
                        new_piece.shift(&self.board, 0, 1);

                        self.current_piece = Some(new_piece);
                    } else {
                        new_piece.place(&mut self.board);

                        self.board.grayize();
                        self.game_over = true;

                        break 'inner;
                    }

                    self.gravity_delta = 0.0;
                    self.move_delay = self.config.das;
                    self.lock_delta = 0.0;
                    self.lock_force_delta = 0.0;
                    self.lock_enabled = false;
                }
            }

            if let Some(piece) = &mut self.current_piece {
                // Move
                let direction = self.player.horizontal();

                if self.move_direction != direction {
                    self.move_direction = direction;

                    if direction != 0 {
                        self.move_delay = self.config.das;

                        if piece.shift(&self.board, self.move_direction, 0) {
                            self.lock_delta = 0.0;
    
                            if self.lock_enabled && !piece.test(&self.board, 0, -1) {
                                events.push(EventType::LockReset);
                            }
                        }
                    }
                }

                if self.move_direction != 0 {
                    let mut moved = false;

                    self.move_delay -= dt;

                    while self.move_delay <= 0.0 {
                        if piece.shift(&self.board, self.move_direction, 0) {
                            self.move_delay += self.config.arr;

                            moved = true;
                        } else {
                            self.move_delay = 0.0;

                            break;
                        }
                    }

                    if moved {
                        self.lock_delta = 0.0;

                        if self.lock_enabled && !piece.test(&self.board, 0, -1) {
                            events.push(EventType::LockReset);
                        }
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
                        events.append(&mut self.place_piece());

                        break 'inner;
                    }

                    self.lock_enabled = true;
                    self.gravity_delta = 0.0;
                } else {
                    self.lock_delta = 0.0;
                }

                // Soft drop
                if self.player.soft_drop() {
                    if self.softdroping {
                        self.softdrop_delay -= dt;
                    } else {
                        self.softdrop_delay = 0.0;
                        self.softdroping = true;
                    }

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
                } else {
                    self.softdroping = false;
                }

                // Rotate
                if self.player.cw() {
                    if piece.cw(&self.board) {
                        self.lock_delta = 0.0;

                        if self.lock_enabled && !piece.test(&self.board, 0, -1) {
                            events.push(EventType::LockReset);
                        }
                    }
                }

                if self.player.ccw() {
                    if piece.ccw(&self.board) {
                        self.lock_delta = 0.0;

                        if self.lock_enabled && !piece.test(&self.board, 0, -1) {
                            events.push(EventType::LockReset);
                        }
                    }
                }

                if self.player.flip() {
                    if piece.flip(&self.board) {
                        self.lock_delta = 0.0;

                        if self.lock_enabled && !piece.test(&self.board, 0, -1) {
                            events.push(EventType::LockReset);
                        }
                    }
                }

                // Hard drop
                if self.player.hard_drop() {
                    while piece.shift(&self.board, 0, -1) {
                        self.score += 2;
                    }

                    events.append(&mut self.place_piece());
                }
            }

            break;
        }

        events
    }

    fn fill_next(&mut self) {
        while self.bag.len() < MINIMUM_NEXT_COUNT {
            self.bag.insert(self.bag.len(), self.randomizer.pop());
        }
    }

    fn place_piece(&mut self) -> Vec<EventType> {
        let mut events = Vec::new();

        if let Some(piece) = &mut self.current_piece {
            piece.place(&mut self.board);

            let lines = self.board.process_lines();
            let cleared = self.board.is_cleared();
            let mut points_and_b2b = Self::calc_points_and_b2b(self.level, lines, cleared, piece.tspin_state());

            if lines >= 1 {
                if points_and_b2b.1 {
                    self.b2b_combo += 1;
    
                    if self.b2b_combo >= 2 {
                        points_and_b2b.0 = points_and_b2b.0 * 3 / 2;
                    }
                } else {
                    self.b2b_combo = 0;
                }

                if self.combo > 0 {
                    points_and_b2b.0 += 50 * self.combo * self.level;
                }

                self.combo += 1;
            } else {
                self.combo = 0;
            }

            self.score += points_and_b2b.0;
            self.lines += lines;
            
            if lines >= 1 || piece.tspin_state() != TSpinType::None {
                events.push(EventType::Pointed {
                    score: points_and_b2b.0,
                    combo: self.combo,
                    lines,
                    b2b: self.b2b_combo >= 2,
                    tspin: piece.tspin_state(),
                });
            }

            while self.level_data.lines > 0 && self.lines - self.lines_offset >= self.level_data.lines {
                self.lines_offset += self.level_data.lines;
                self.level += 1;
                self.level_data = self.config.levels[(self.level - 1) as usize];
            }

            self.current_piece = None;
            self.hold_enabled = true;
            self.move_delay = 0.0;
        }

        events
    }

    fn calc_points_and_b2b(level: i32, lines: i32, cleared: bool, tspin_state: TSpinType) -> (i32, bool) {
        return match (lines, tspin_state, cleared) {
            (1, TSpinType::None, true) => (800 * level, false),
            (1, TSpinType::None, false) => (100 * level, false),
            (2, TSpinType::None, true) => (1200 * level, false),
            (2, TSpinType::None, false) => (300 * level, false),
            (3, TSpinType::None, true) => (1800 * level, false),
            (3, TSpinType::None, false) => (500 * level, false),
            (4, TSpinType::None, true) => (2000 * level, true),
            (4, TSpinType::None, false) => (800 * level, true),
            (0, TSpinType::Normal, _) => (400 * level, false),
            (1, TSpinType::Normal, _) => (800 * level, true),
            (2, TSpinType::Normal, _) => (1200 * level, true),
            (3, TSpinType::Normal, _) => (1600 * level, true),
            (0, TSpinType::Mini, _) => (100 * level, false),
            (1, TSpinType::Mini, _) => (200 * level, true),
            (2, TSpinType::Mini, _) => (400 * level, true),
            _ => (0, false),
        };
    }
}
