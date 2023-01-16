use macroquad::audio::play_sound_once;
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

pub struct Board {
    session: Game,
    state: State,
    state_time: f32,
    board_scale: f32,
}

impl Board {
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

    #[cfg(debug_assertions)]
    fn update_debug(&mut self) {
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
}

impl Object for Board {
    fn init(&mut self) {
        play_sound_once(sound("ready"));
    }
    
    fn update(&mut self) {
        let dt = get_frame_time();

        self.state_time += dt;
        self.board_scale = (self.board_scale + dt / 0.5).min(1.0);

        match self.state {
            State::Ready => {
                if self.state_time >= 2.0 {
                    play_sound_once(sound("go"));
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
                        EventType::Move => {
                            play_sound_once(sound("move"));
                        },
                        EventType::Rotate { is_spin } => {
                            play_sound_once(sound(match is_spin {
                                true => "rotate_spin",
                                false => "rotate",
                            }));
                        },
                        EventType::HardDrop => {
                            play_sound_once(sound("hard_drop"));
                        },
                        EventType::Lock => {
                            play_sound_once(sound("lock"));
                        },
                        EventType::Hold => {
                            play_sound_once(sound("hold"))
                        },
                        EventType::LineClear { score: _, lines, combo, b2b, tspin } => {
                            let draw_left = self.draw_left();
                            let draw_right = self.draw_right();
                            let draw_bottom = self.draw_bottom();
                            let draw_top = self.draw_top();

                            let message = String::from(match (b2b, lines, tspin) {
                                (false, 1, TSpinType::None) => "SINGLE",
                                (false, 2, TSpinType::None) => "DOUBLE",
                                (false, 3, TSpinType::None) => "TRIPLE",
                                (false, 4, TSpinType::None) => "QUAD",
                                (false, 1, TSpinType::Normal) => "T-SPIN SINGLE",
                                (false, 2, TSpinType::Normal) => "T-SPIN DOUBLE",
                                (false, 3, TSpinType::Normal) => "T-SPIN TRIPLE",
                                (false, 1, TSpinType::Mini) => "T-SPIN MINI SINGLE",
                                (false, 2, TSpinType::Mini) => "T-SPIN MINI DOUBLE",
                                (true, 4, TSpinType::None) => "B2B QUAD",
                                (true, 1, TSpinType::Normal) => "B2B T-SPIN SINGLE",
                                (true, 2, TSpinType::Normal) => "B2B T-SPIN DOUBLE",
                                (true, 3, TSpinType::Normal) => "B2B T-SPIN TRIPLE",
                                (true, 1, TSpinType::Mini) => "B2B T-SPIN MINI SINGLE",
                                (true, 2, TSpinType::Mini) => "B2B T-SPIN MINI DOUBLE",
                                (_, 0, TSpinType::Normal) => "T-SPIN",
                                (_, 0, TSpinType::Mini) => "T-SPIN MINI",
                                _ => "",
                            });

                            let sub_message = match combo {
                                _ if combo >= 2 => format!("{} COMBO", combo),
                                _ => String::new(),
                            };

                            'a: {
                                play_sound_once(sound(match (lines, tspin) {
                                    (1 | 2 | 3, TSpinType::None) => "erase",
                                    (4, TSpinType::None) => "erase_quad",
                                    (1 | 2 | 3, TSpinType::Normal) | (1 | 2, TSpinType::Mini) => "tspin",
                                    _ => break 'a,
                                }));
                            }

                            object_add(-1, PointMessage::new((draw_left + draw_right) * 0.5, (draw_top + draw_bottom) * 0.5, message, sub_message));
                        },
                        EventType::LevelUp => {
                            play_sound_once(sound("level_up"));
                        }
                        EventType::GameOver => {
                            play_sound_once(sound("game_over"));
                        },
                    }
                }
            },
        }

        self.update_debug();
    }

    fn draw(&self) {
        // Scale matrix
        push_matrix_trs(0.0, 0.0, 0.0, simple_easing::bounce_out(self.board_scale), 1.0);

        // Positions
        let draw_left = self.draw_left();
        let draw_right = self.draw_right();
        let draw_bottom = self.draw_bottom();
        let draw_top = self.draw_top();

        // Textures
        let block_texture = &texture("blocks");

        // Board
        draw_panel(draw_left, draw_top, draw_right - draw_left, draw_bottom - draw_top);

        match self.state {
            State::Ready => {
                draw_text_aligned("READY?", (draw_left + draw_right) * 0.5, (draw_top + draw_bottom) * 0.5, font_default(), 38, 0.5, 0.5, colors::WHITE);
            },
            State::Ingame => {
                for y in 0..self.session.board.row_count() as i32 {
                    for x in 0..self.session.board.width() as i32 {
                        let position = calc_block_position(draw_left, draw_bottom, x, y, CELL_SIZE);
        
                        draw_block(block_texture, position.0, position.1, CELL_SIZE, self.session.board.get(x, y), 1.0);
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
        
                            draw_block(block_texture, position.0, position.1, CELL_SIZE, piece.block_at(x, y), 0.3);
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
        
                            draw_block(block_texture, position.0, position.1, CELL_SIZE, block, 1.0);
        
                            if block != BlockType::Empty && lock_flash > 0.0 {
                                gl_use_material(material_additive());
                                draw_rectangle(position.0, position.1, CELL_SIZE, CELL_SIZE, Color::new(1.0, 1.0, 1.0, (lock_flash - 0.4).max(0.0) / 0.6 * 0.35));
                                gl_use_default_material();
                            }
                        }
                    }
                }        
            },
        }
        
        // Hold
        draw_text_aligned("HOLD", draw_left - 16.0, draw_top - 1.0, font_default(), 22, 1.0, 0.0, colors::WHITE);
        draw_panel(draw_left - 121.0, draw_top + 29.0, 102.0, 82.0);

        if let Some(hold_piece) = self.session.hold_piece {
            draw_preview(block_texture, draw_left - 70.0, draw_top + 70.0, PREVIEW_CELL_SIZE, &self.session.rotation, hold_piece, match self.session.hold_enabled {
                true => 1.0,
                false => 0.5,
            });
        }

        // Next
        draw_text_aligned("NEXT", draw_right + 16.0, draw_top - 1.0, font_default(), 22, 0.0, 0.0, colors::WHITE);
        draw_panel(draw_right + 19.0, draw_top + 29.0, 102.0, 322.0);

        for i in 0..5 {
            draw_preview(block_texture, draw_right + 70.0, draw_top + 70.0 + i as f32 * 60.0, PREVIEW_CELL_SIZE, &self.session.rotation, self.session.bag[i], 1.0);
        }

        // Statuses
        draw_text_aligned("LEVEL", draw_left - 16.0, draw_top + 136.0, font_default(), 22, 1.0, 0.0, colors::WHITE);
        draw_text_aligned(self.session.level.to_string().as_str(), draw_left - 15.0, draw_top + 166.0, font_default(), 38, 1.0, 0.0, colors::WHITE);

        draw_text_aligned("LINES", draw_left - 16.0, draw_top + 216.0, font_default(), 22, 1.0, 0.0, colors::WHITE);
        draw_text_aligned(self.session.lines.to_string().as_str(), draw_left - 15.0, draw_top + 246.0, font_default(), 38, 1.0, 0.0, colors::WHITE);

        draw_text_aligned("SCORE", draw_left - 16.0, draw_top + 296.0, font_default(), 22, 1.0, 0.0, colors::WHITE);
        draw_text_aligned(self.session.score.to_string().as_str(), draw_left - 15.0, draw_top + 326.0, font_default(), 38, 1.0, 0.0, colors::WHITE);

        pop_matrix();
    }
}