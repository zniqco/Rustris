use super::*;

const MINIMUM_NEXT_COUNT: usize = 7;

pub enum EventType {
    Move,
    Rotate { is_spin: bool },
    HardDrop,
    Lock,
    Hold,
    LineClear { score: i32, lines: i32, combo: i32, b2b: bool, tspin: TSpinState },
    LevelUp,
    GameOver,
}

pub struct Game {
    pub input: Input,
    das: f32,
    arr: f32,
    sdf: f32,
    mode: Mode,
    pub board: Board,
    score: i64,
    level: i32,
    lines: i32,
    pub current_piece: Option<Piece>,
    pub bag: Vec<PieceType>,
    randomizer: Randomizer,
    pub rotation: Rotation,
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
    pub fn new(config: Config, mode: Mode, seed: u64) -> Self {
        let width = mode.width;
        let height = mode.height;
        let first_level = mode.levels[0];
        let mut instance = Self {
            input: Input::new(),
            das: mode.das.unwrap_or(config.das),
            arr: mode.arr.unwrap_or(config.arr),
            sdf: mode.sdf.unwrap_or(config.sdf),
            mode,
            board: Board::new(width, height),
            score: 0,
            level: 1,
            lines: 0,
            current_piece: None,
            bag: Vec::new(),
            randomizer: mode.randomizer.to_struct(seed),
            rotation: mode.rotation.unwrap_or(config.rotation).to_struct(),
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

    pub fn score(&self) -> i64 {
        self.score
    }

    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn lines(&self) -> i32 {
        self.lines
    }

    pub fn update(&mut self, dt: f32) -> Vec<EventType> {
        let mut events = Vec::new();

        self.input.update();

        'inner: loop {
            if !self.game_over {
                if self.input.hold() && self.hold_enabled {
                    if let Some(piece) = &mut self.current_piece {
                        if let Some(hold_piece) = self.hold_piece {
                            self.bag.insert(0, hold_piece);
                        }

                        self.hold_piece = Some(piece.piece_type());
                        self.current_piece = None;
                        self.hold_enabled = false;

                        events.push(EventType::Hold);
                    }
                }

                if self.current_piece.is_none() {
                    let piece_type = self.bag[0];
                    let piece_data = self.rotation.piece(piece_type);
                    let spawn_x = self.board.width() as i32 / 2 + piece_data.spawn_offset.0;
                    let spawn_y = self.board.height() as i32 + piece_data.spawn_offset.1;
                    let mut new_piece = Piece::new(piece_type, piece_data, (spawn_x, spawn_y));

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

                        events.push(EventType::GameOver);

                        break 'inner;
                    }

                    self.gravity_delta = 0.0;
                    self.move_delay = self.das;
                    self.lock_delta = 0.0;
                    self.lock_force_delta = 0.0;
                    self.lock_enabled = false;
                }
            }

            if let Some(piece) = &mut self.current_piece {
                // Move
                let direction = self.input.horizontal();

                if self.move_direction != direction {
                    self.move_direction = direction;

                    if direction != 0 {
                        self.move_delay = self.das;

                        if piece.shift(&self.board, self.move_direction, 0) {
                            self.lock_delta = 0.0;
                            events.push(EventType::Move);
                        }
                    }
                }

                if self.move_direction != 0 {
                    let mut moved = false;

                    self.move_delay -= dt;

                    while self.move_delay <= 0.0 {
                        if piece.shift(&self.board, self.move_direction, 0) {
                            self.move_delay += self.arr;

                            moved = true;
                        } else {
                            self.move_delay = 0.0;

                            break;
                        }
                    }

                    if moved {
                        events.push(EventType::Move);
                        self.lock_delta = 0.0;
                    }
                }

                // Gravity
                self.gravity_delta += self.level_data.gravity * 60.0 * dt;

                while self.gravity_delta >= 1.0 {
                    if piece.shift(&self.board, 0, -1) {
                        self.gravity_delta -= 1.0;
                    } else {
                        self.gravity_delta = 0.0;
                        break;
                    }
                }
                
                if self.lock_enabled {
                    self.lock_force_delta += dt / self.level_data.lock_delay / 4.0;
                }

                if !piece.test(&self.board, 0, -1) {
                    self.lock_delta += dt / self.level_data.lock_delay;

                    if self.lock_delta >= 1.0 || self.lock_force_delta >= 1.0 {
                        events.push(EventType::Lock);
                        events.append(&mut self.place_piece());

                        break 'inner;
                    }

                    self.lock_enabled = true;
                    self.gravity_delta = 0.0;
                } else {
                    self.lock_delta = 0.0;
                }

                // Soft drop
                if self.input.soft_drop() {
                    if self.softdroping {
                        self.softdrop_delay -= dt;
                    } else {
                        self.softdrop_delay = 0.0;
                        self.softdroping = true;
                    }

                    while self.softdrop_delay <= 0.0 {
                        if piece.shift(&self.board, 0, -1) {
                            self.score += 1;
                            self.softdrop_delay += self.sdf;
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
                let mut rotated = false;

                if self.input.cw() && piece.cw(&self.board) {
                    self.lock_delta = 0.0;
                    rotated = true;
                }

                if self.input.ccw() && piece.ccw(&self.board) {
                    self.lock_delta = 0.0;
                    rotated = true;
                }

                if self.input.flip() && piece.flip(&self.board) {
                    self.lock_delta = 0.0;
                    rotated = true;
                }

                if rotated {
                    events.push(EventType::Rotate {
                        is_spin: matches!(piece.tspin_state(), TSpinState::Normal | TSpinState::Mini)
                    });
                }

                // Hard drop
                if self.input.hard_drop() {
                    while piece.shift(&self.board, 0, -1) {
                        self.score += 2;
                    }

                    events.push(EventType::HardDrop);
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

            self.score += points_and_b2b.0 as i64;
            self.lines += lines;
            
            if lines >= 1 || piece.tspin_state() != TSpinState::None {
                events.push(EventType::LineClear {
                    score: points_and_b2b.0,
                    combo: self.combo,
                    lines,
                    b2b: self.b2b_combo >= 2,
                    tspin: piece.tspin_state(),
                });
            }

            let mut level_up = false;

            while self.level_data.lines > 0 && self.lines - self.lines_offset >= self.level_data.lines {
                self.lines_offset += self.level_data.lines;
                self.level += 1;
                self.level_data = self.mode.levels[(self.level - 1) as usize];

                level_up = true;
            }

            if level_up {
                events.push(EventType::LevelUp);
            }

            self.current_piece = None;
            self.hold_enabled = true;
            self.move_delay = 0.0;
        }

        events
    }

    fn calc_points_and_b2b(level: i32, lines: i32, cleared: bool, tspin_state: TSpinState) -> (i32, bool) {
        match (lines, tspin_state, cleared) {
            (1, TSpinState::None, true) => (800 * level, false),
            (1, TSpinState::None, false) => (100 * level, false),
            (2, TSpinState::None, true) => (1200 * level, false),
            (2, TSpinState::None, false) => (300 * level, false),
            (3, TSpinState::None, true) => (1800 * level, false),
            (3, TSpinState::None, false) => (500 * level, false),
            (4, TSpinState::None, true) => (2000 * level, true),
            (4, TSpinState::None, false) => (800 * level, true),
            (0, TSpinState::Normal, _) => (400 * level, false),
            (1, TSpinState::Normal, _) => (800 * level, true),
            (2, TSpinState::Normal, _) => (1200 * level, true),
            (3, TSpinState::Normal, _) => (1600 * level, true),
            (0, TSpinState::Mini, _) => (100 * level, false),
            (1, TSpinState::Mini, _) => (200 * level, true),
            (2, TSpinState::Mini, _) => (400 * level, true),
            _ => (0, false),
        }
    }
}
