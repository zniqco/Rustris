use macroquad::prelude::*;
use macroquad::color::colors;
use crate::game::*;
use super::super::*;

const CELL_SIZE: f32 = 30.0;
const PREVIEW_CELL_SIZE: f32 = 20.0;

enum State {
    Ready,
    Ingame,
}

pub struct Ingame {
    session: Game,
    state: State,
    state_time: f32,
    board_scale: f32,
}

impl Ingame {
    pub fn new() -> Self {
        Self {
            session: Game::new(Config {
                ..Default::default()
            }),
            state: State::Ready,
            state_time: 0.0,
            board_scale: 0.0,
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        self.state_time += dt;
        self.board_scale = (self.board_scale + dt / 0.5).min(1.0);

        match self.state {
            State::Ready => {
                if self.state_time >= 2.0 {
                    self.state = State::Ingame;
                }
            },
            State::Ingame => {
                self.session.input.set(InputType::MoveLeft, is_key_down(KeyCode::Left));
                self.session.input.set(InputType::MoveRight, is_key_down(KeyCode::Right));
                self.session.input.set(InputType::SoftDrop, is_key_down(KeyCode::Down));
                self.session.input.set(InputType::HardDrop, is_key_down(KeyCode::Space));
                self.session.input.set(InputType::RotateCW, is_key_down(KeyCode::Up) || is_key_down(KeyCode::X));
                self.session.input.set(InputType::RotateCCW, is_key_down(KeyCode::Z));
                self.session.input.set(InputType::Flip, is_key_down(KeyCode::A));
                self.session.input.set(InputType::Hold, is_key_down(KeyCode::C));

                for event in self.session.update(dt) {
                    match event {
                        EventType::Pointed { score, lines, combo, tspin } => {
                            println!("Pointed (score={}, lines={}, combo={}, tspin={:?})", score, lines, combo, tspin);
                        },
                        _ => { }
                    }
                }
            },
        }
    }

    pub fn draw(&self) {
        // Scale matrix
        let scale = screen_height() / 720.0;

        push_matrix_trs(screen_width() * 0.5, screen_height() * 0.5, 0.0, simple_easing::bounce_out(self.board_scale) * scale, scale);

        // Positions
        let board_width = self.session.board.width() as i32;
        let board_height = self.session.board.height() as i32;
        let draw_left = CELL_SIZE * board_width as f32 * -0.5;
        let draw_right = CELL_SIZE * board_width as f32 * 0.5;
        let draw_bottom = CELL_SIZE * board_height as f32 * 0.5;
        let draw_top = CELL_SIZE * board_height as f32 * -0.5;

        // Board
        draw_panel(draw_left, draw_top, board_width as f32 * CELL_SIZE, board_height as f32 * CELL_SIZE);

        match self.state {
            State::Ready => {
                draw_text_aligned("Ready?", 0.0, 0.0, *DEFAULT_FONT, 42, 0.5, 0.5, colors::WHITE);
            },
            State::Ingame => {
                for y in 0..self.session.board.row_count() as i32 {
                    for x in 0..board_width {
                        let position = get_block_position(draw_left, draw_bottom, x, y, CELL_SIZE);
        
                        draw_block(position.0, position.1, CELL_SIZE, self.session.board.get_block(x, y), 1.0);
                    }
                }
        
                if let Some(piece) = &self.session.current_piece {
                    let mut ghost_offset = 0;
        
                    while piece.test(&self.session.board, 0, ghost_offset - 1) {
                        ghost_offset -= 1;
                    }
        
                    for y in 0..4 {
                        for x in 0..4 {
                            let position = get_block_position(draw_left, draw_bottom, x + piece.x, y + piece.y + ghost_offset, CELL_SIZE);
        
                            draw_block(position.0, position.1, CELL_SIZE, piece.get_block(x, y), 0.3);
                        }
                    }
        
                    let floating = piece.test(&self.session.board, 0, -1);
                    let lock_flash = match floating {
                        true => 0.0,
                        false => (1.0 - self.session.lock_delta) * (1.0 - self.session.lock_force_delta),
                    };
        
                    for y in 0..4 {
                        for x in 0..4 {
                            let position = get_block_position(draw_left, draw_bottom, x + piece.x, y + piece.y, CELL_SIZE);
                            let block = piece.get_block(x, y);
        
                            draw_block(position.0, position.1, CELL_SIZE, block, 1.0);
        
                            if block != BlockType::Empty && lock_flash > 0.0 {
                                gl_use_material(*ADDITIVE_MATERIAL);
                                draw_rectangle(position.0, position.1, CELL_SIZE, CELL_SIZE, Color::new(1.0, 1.0, 1.0, (lock_flash - 0.4).max(0.0) / 0.6 * 0.35));
                                gl_use_default_material();
                            }
                        }
                    }
                }        
            },
        }
        
        // Hold
        draw_text_aligned("Hold", draw_left - 16.0, draw_top - 1.0, *DEFAULT_FONT, 22, 1.0, 0.0, colors::WHITE);
        draw_panel(draw_left - 121.0, draw_top + 29.0, 102.0, 82.0);

        if let Some(hold_piece) = self.session.hold_piece {
            draw_preview(draw_left - 70.0, draw_top + 70.0, PREVIEW_CELL_SIZE, hold_piece, match self.session.hold_enabled {
                true => 1.0,
                false => 0.5,
            });
        }

        // Next
        draw_text_aligned("Next", draw_right + 16.0, draw_top - 1.0, *DEFAULT_FONT, 22, 0.0, 0.0, colors::WHITE);
        draw_panel(draw_right + 19.0, draw_top + 29.0, 102.0, 322.0);

        for i in 0..5 {
            draw_preview(draw_right + 70.0, draw_top + 70.0 + i as f32 * 60.0, PREVIEW_CELL_SIZE, self.session.bag.get(i), 1.0);
        }

        // Statuses
        draw_text_aligned("Level", draw_left - 16.0, draw_top + 136.0, *DEFAULT_FONT, 22, 1.0, 0.0, colors::WHITE);
        draw_text_aligned(self.session.level.to_string().as_str(), draw_left - 15.0, draw_top + 166.0, *DEFAULT_FONT, 42, 1.0, 0.0, colors::WHITE);

        draw_text_aligned("Lines", draw_left - 16.0, draw_top + 216.0, *DEFAULT_FONT, 22, 1.0, 0.0, colors::WHITE);
        draw_text_aligned(self.session.lines.to_string().as_str(), draw_left - 15.0, draw_top + 246.0, *DEFAULT_FONT, 42, 1.0, 0.0, colors::WHITE);

        draw_text_aligned("Score", draw_left - 16.0, draw_top + 296.0, *DEFAULT_FONT, 22, 1.0, 0.0, colors::WHITE);
        draw_text_aligned(self.session.score.to_string().as_str(), draw_left - 15.0, draw_top + 326.0, *DEFAULT_FONT, 42, 1.0, 0.0, colors::WHITE);

        pop_matrix();
    }
}