use macroquad::prelude::*;
use macroquad::color::colors;
use crate::game::*;
use super::*;

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

    fn draw_left(&self) -> f32 {
        CELL_SIZE * self.session.board.width() as f32 * -0.5
    }

    fn draw_right(&self) -> f32 {
        CELL_SIZE * self.session.board.width() as f32 * 0.5
    }

    fn draw_top(&self) -> f32 {
        CELL_SIZE * self.session.board.height() as f32 * -0.5
    }

    fn draw_bottom(&self) -> f32 {
        CELL_SIZE * self.session.board.height() as f32 * 0.5
    }
}

impl Object for Ingame {
    fn update(&mut self) -> Vec<ObjectEvent> {
        let mut events = Vec::new();
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
                match self.session.player {
                    PlayerType::User(ref mut user) => {
                        user.move_left = is_key_down(KeyCode::Left);
                        user.move_right = is_key_down(KeyCode::Right);
                        user.soft_drop = is_key_down(KeyCode::Down);
                        user.hard_drop = is_key_down(KeyCode::Space);
                        user.rotate_cw = is_key_down(KeyCode::Up) || is_key_down(KeyCode::X);
                        user.rotate_ccw = is_key_down(KeyCode::Z);
                        user.flip = is_key_down(KeyCode::A);
                        user.hold = is_key_down(KeyCode::C);
                    },
                    _ => { }
                }

                for event in self.session.update(dt) {
                    match event {
                        EventType::Pointed { score: _, lines, combo, b2b, tspin } => {
                            let draw_left = self.draw_left();
                            let draw_right = self.draw_right();
                            let draw_bottom = self.draw_bottom();
                            let draw_top = self.draw_top();

                            let message = String::from(match (b2b, lines, tspin) {
                                (false, 1, TSpinType::None) => "Single",
                                (false, 2, TSpinType::None) => "Double",
                                (false, 3, TSpinType::None) => "Triple",
                                (false, 4, TSpinType::None) => "Tetris",
                                (false, 1, TSpinType::Normal) => "T-Spin Single",
                                (false, 2, TSpinType::Normal) => "T-Spin Double",
                                (false, 3, TSpinType::Normal) => "T-Spin Triple",
                                (false, 1, TSpinType::Mini) => "T-Spin Mini Single",
                                (false, 2, TSpinType::Mini) => "T-Spin Mini Double",
                                (true, 4, TSpinType::None) => "B2B Tetris",
                                (true, 1, TSpinType::Normal) => "B2B T-Spin Single",
                                (true, 2, TSpinType::Normal) => "B2B T-Spin Double",
                                (true, 3, TSpinType::Normal) => "B2B T-Spin Triple",
                                (true, 1, TSpinType::Mini) => "B2B T-Spin Mini Single",
                                (true, 2, TSpinType::Mini) => "B2B T-Spin Mini Double",
                                (_, 0, TSpinType::Normal) => "T-Spin",
                                (_, 0, TSpinType::Mini) => "T-Spin Mini",
                                _ => "",
                            });

                            let sub_message = match combo {
                                _ if combo >= 2 => format!("{} combo", combo),
                                _ => String::new(),
                            };

                            events.push(ObjectEvent::Create { 
                                depth: 0,
                                object: PointMessage::new((draw_left + draw_right) * 0.5, (draw_top + draw_bottom) * 0.5, message, sub_message).into()
                            });
                        },
                        _ => { }
                    }
                }
            },
        }

        if cfg!(debug_assertions) {
            if is_mouse_button_down(MouseButton::Left) || is_mouse_button_down(MouseButton::Right) {
                let mouse_position = mouse_position();
                let draw_left = self.draw_left() + screen_width() * 0.5;
                let draw_bottom = self.draw_bottom() + screen_height() * 0.5;
                let x = ((mouse_position.0 - draw_left) / CELL_SIZE).floor() as i32;
                let y = ((draw_bottom - mouse_position.1) / CELL_SIZE).floor() as i32;

                if is_mouse_button_down(MouseButton::Left) {
                    self.session.board.set(x, y, BlockType::Gray);
                } else {
                    self.session.board.set(x, y, BlockType::Empty);
                }
            }
        }

        events
    }

    fn draw(&self) {
        // Scale matrix
        push_matrix_trs(0.0, 0.0, 0.0, simple_easing::bounce_out(self.board_scale), 1.0);

        // Positions
        let draw_left = self.draw_left();
        let draw_right = self.draw_right();
        let draw_bottom = self.draw_bottom();
        let draw_top = self.draw_top();

        // Board
        draw_panel(draw_left, draw_top, draw_right - draw_left, draw_bottom - draw_top);

        match self.state {
            State::Ready => {
                draw_text_aligned("Ready?", (draw_left + draw_right) * 0.5, (draw_top + draw_bottom) * 0.5, *DEFAULT_FONT, 42, 0.5, 0.5, colors::WHITE);
            },
            State::Ingame => {
                for y in 0..self.session.board.row_count() as i32 {
                    for x in 0..self.session.board.width() as i32 {
                        let position = calc_block_position(draw_left, draw_bottom, x, y, CELL_SIZE);
        
                        draw_block(position.0, position.1, CELL_SIZE, self.session.board.get(x, y), 1.0);
                    }
                }
        
                if let Some(piece) = &self.session.current_piece {
                    let mut ghost_offset = 0;
        
                    while piece.test(&self.session.board, 0, ghost_offset - 1) {
                        ghost_offset -= 1;
                    }
        
                    for y in 0..piece.height() {
                        for x in 0..piece.width() {
                            let position = calc_block_position(draw_left, draw_bottom, x + piece.x(), y + piece.y() + ghost_offset, CELL_SIZE);
        
                            draw_block(position.0, position.1, CELL_SIZE, piece.block_at(x, y), 0.3);
                        }
                    }
        
                    let floating = piece.test(&self.session.board, 0, -1);
                    let lock_flash = match floating {
                        true => 0.0,
                        false => (1.0 - self.session.lock_delta) * (1.0 - self.session.lock_force_delta),
                    };
        
                    for y in 0..piece.height() {
                        for x in 0..piece.width() {
                            let position = calc_block_position(draw_left, draw_bottom, x + piece.x(), y + piece.y(), CELL_SIZE);
                            let block = piece.block_at(x, y);
        
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
            draw_preview(draw_left - 70.0, draw_top + 70.0, PREVIEW_CELL_SIZE, &self.session.rotation, hold_piece, match self.session.hold_enabled {
                true => 1.0,
                false => 0.5,
            });
        }

        // Next
        draw_text_aligned("Next", draw_right + 16.0, draw_top - 1.0, *DEFAULT_FONT, 22, 0.0, 0.0, colors::WHITE);
        draw_panel(draw_right + 19.0, draw_top + 29.0, 102.0, 322.0);

        for i in 0..5 {
            draw_preview(draw_right + 70.0, draw_top + 70.0 + i as f32 * 60.0, PREVIEW_CELL_SIZE, &self.session.rotation, self.session.bag[i], 1.0);
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