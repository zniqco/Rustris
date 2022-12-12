use macroquad::prelude::*;
use macroquad::color::colors;
use crate::extension::*;
use crate::game::*;

pub struct Rustris {
    session: Game,
    background_texture: Texture2D,
    blocks_texture: Texture2D,
    font: Font,
}

const CELL_SIZE: f32 = 30.0;
const PREVIEW_CELL_SIZE: f32 = 20.0;

impl Rustris {
    pub fn new() -> Self {
        Self {
            session: Game::new(Config {
                ..Default::default()
            }),
            background_texture: Texture2D::from_file_with_format(include_bytes!("../../assets/background.png"), None),
            blocks_texture: Texture2D::from_file_with_format(include_bytes!("../../assets/blocks.png"), None),
            font: load_ttf_font_from_bytes(include_bytes!("../../assets/font.ttf")).unwrap(),
        }
    }

    pub fn update(&mut self) {
        self.session.input.set(InputType::MoveLeft, is_key_down(KeyCode::Left));
        self.session.input.set(InputType::MoveRight, is_key_down(KeyCode::Right));
        self.session.input.set(InputType::SoftDrop, is_key_down(KeyCode::Down));
        self.session.input.set(InputType::HardDrop, is_key_down(KeyCode::Space));
        self.session.input.set(InputType::RotateCW, is_key_down(KeyCode::Up) || is_key_down(KeyCode::X));
        self.session.input.set(InputType::RotateCCW, is_key_down(KeyCode::Z));
        self.session.input.set(InputType::Flip, is_key_down(KeyCode::A));
        self.session.input.set(InputType::Hold, is_key_down(KeyCode::C));

        self.session.update(get_frame_time());
    }

    pub fn draw(&self) {
        clear_background(colors::BLACK);

        draw_texture_ex(self.background_texture, 0.0, 0.0, colors::WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(screen_width(), screen_height())),
            ..Default::default()
        });

        let board_width = self.session.board.width() as i32;
        let board_height = self.session.board.height() as i32;
        let draw_left = (screen_width() - CELL_SIZE * board_width as f32) / 2.0;
        let draw_right = draw_left + CELL_SIZE * board_width as f32;
        let draw_bottom = (screen_height() + CELL_SIZE * board_height as f32) / 2.0;
        let draw_top = draw_bottom - CELL_SIZE * board_height as f32;

        // Board
        self.draw_panel(draw_left, draw_top, board_width as f32 * CELL_SIZE, board_height as f32 * CELL_SIZE);

        for y in 0..self.session.board.row_count() as i32 {
            for x in 0..board_width {
                self.draw_block(draw_left, draw_bottom, x, y, self.session.board.get_block(x, y), 1.0, 0.0);
            }
        }

        if let Some(piece) = &self.session.current_piece {
            let mut ghost_offset = 0;

            while piece.test(&self.session.board, 0, ghost_offset - 1) {
                ghost_offset -= 1;
            }

            for y in 0..4 {
                for x in 0..4 {
                    self.draw_block(draw_left, draw_bottom, x + piece.x, y + piece.y + ghost_offset, piece.get_block(x, y), 0.3, 0.0);
                }
            }

            let floating = piece.test(&self.session.board, 0, -1);

            for y in 0..4 {
                for x in 0..4 {
                    self.draw_block(draw_left, draw_bottom, x + piece.x, y + piece.y, piece.get_block(x, y), 1.0, match floating {
                        true => 0.0,
                        false => (1.0 - self.session.lock_delta) * (1.0 - self.session.lock_force_delta),
                    });
                }
            }
        }

        // Hold
        draw_text_aligned("Hold", draw_left - 16.0, draw_top + 26.0, self.font, 24, 1.0, 0.5, colors::WHITE);
        self.draw_panel(draw_left - 121.0, draw_top + 29.0, 102.0, 82.0);

        if let Some(hold_piece) = self.session.hold_piece {
            self.draw_preview(draw_left - 70.0, draw_top + 70.0, hold_piece);
        }

        // Next
        draw_text_aligned("Next", draw_right + 16.0, draw_top + 26.0, self.font, 24, 0.0, 0.5, colors::WHITE);
        self.draw_panel(draw_right + 19.0, draw_top + 29.0, 102.0, 322.0);

        for i in 0..5 {
            self.draw_preview(draw_right + 70.0, draw_top + 70.0 + i as f32 * 60.0, self.session.bag.get(i));
        }

        // Statuses
        draw_text_aligned("Level", draw_left - 16.0, draw_top + 160.0, self.font, 24, 1.0, 0.5, colors::WHITE);
        draw_text_aligned(self.session.level.to_string().as_str(), draw_left - 15.0, draw_top + 206.0, self.font, 42, 1.0, 0.5, colors::WHITE);

        draw_text_aligned("Lines", draw_left - 16.0, draw_top + 240.0, self.font, 24, 1.0, 0.5, colors::WHITE);
        draw_text_aligned(self.session.lines.to_string().as_str(), draw_left - 15.0, draw_top + 286.0, self.font, 42, 1.0, 0.5, colors::WHITE);

        draw_text_aligned("Score", draw_left - 16.0, draw_top + 320.0, self.font, 24, 1.0, 0.5, colors::WHITE);
        draw_text_aligned(self.session.score.to_string().as_str(), draw_left - 15.0, draw_top + 366.0, self.font, 42, 1.0, 0.5, colors::WHITE);
    }

    fn draw_block(&self, left: f32, bottom: f32, x: i32, y: i32, block_type: BlockType, alpha: f32, flash: f32) {
        match block_type {
            BlockType::Empty | BlockType::Outside => { },
            _ => {
                let draw_x = left + x as f32 * CELL_SIZE;
                let draw_y = bottom - (y + 1) as f32 * CELL_SIZE;

                draw_texture_ex(self.blocks_texture, draw_x, draw_y, Color::new(1.0, 1.0, 1.0, alpha), DrawTextureParams {
                    dest_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    source: Some(Rect::new(block_type as i32 as f32 * 30.0, 0.0, 30.0, 30.0)),
                    ..Default::default()
                });

                if flash > 0.0 {
                    gl_use_material(get_additive_material());

                    let flash_power = (flash - 0.4).max(0.0) / 0.6;

                    draw_rectangle(draw_x, draw_y, CELL_SIZE, CELL_SIZE, Color::new(1.0, 1.0, 1.0, flash_power * 0.35));

                    gl_use_default_material();
                }
            }
        }
    }

    fn draw_preview(&self, x: f32, y: f32, piece_type: PieceType) {
        let array = match piece_type {
            PieceType::Z => [(-1.0, -0.5), (0.0, -0.5), (0.0, 0.5), (1.0, 0.5)],
            PieceType::S => [(1.0, -0.5), (0.0, -0.5), (0.0, 0.5), (-1.0, 0.5)],
            PieceType::L => [(-1.0, 0.5), (0.0, 0.5), (1.0, 0.5), (1.0, -0.5)],
            PieceType::J => [(-1.0, -0.5), (-1.0, 0.5), (0.0, 0.5), (1.0, 0.5)],
            PieceType::I => [(-1.5, 0.0), (-0.5, 0.0), (0.5, 0.0), (1.5, 0.0)],
            PieceType::O => [(-0.5, -0.5), (0.5, -0.5), (-0.5, 0.5), (0.5, 0.5)],
            PieceType::T => [(-1.0, 0.5), (0.0, 0.5), (0.0, -0.5), (1.0, 0.5)],
        };

        for cell in array {
            draw_texture_ex(self.blocks_texture, x + ((cell.0 - 0.5) * PREVIEW_CELL_SIZE), y + ((cell.1 - 0.5) * PREVIEW_CELL_SIZE), colors::WHITE, DrawTextureParams {
                dest_size: Some(Vec2::new(PREVIEW_CELL_SIZE, PREVIEW_CELL_SIZE)),
                source: Some(Rect::new(piece_type.get_block_type() as i32 as f32 * 30.0, 0.0, 30.0, 30.0)),
                ..Default::default()
            });
        }
    }

    fn draw_panel(&self, x: f32, y: f32, width: f32, height: f32) {
        draw_rectangle_lines(x - 2.0, y - 2.0, width + 4.0, height + 4.0, 2.0, colors::WHITE);
        draw_rectangle(x, y, width, height, Color::new(0.0, 0.0, 0.0, 0.875));
    }
}