mod core;
mod raylib_extensions;

use raylib::prelude::*;
use self::core::*;
use self::raylib_extensions::*;

pub struct Rustris {
    screen_width: i32,
    screen_height: i32,
    game: Core,
    background_texture: Option<Texture2D>,
    blocks_texture: Option<Texture2D>,
}

const CELL_SIZE: i32 = 30;
const PREVIEW_CELL_SIZE: i32 = 20;

impl Rustris {
    pub fn new(screen_width: i32, screen_height: i32) -> Self {
        Self {
            screen_width,
            screen_height,
            game: Core::new(Config {
                ..Default::default()
            }),
            background_texture: None,
            blocks_texture: None,
        }
    }

    pub fn init(&mut self, rl: &mut RaylibHandle, rt: &RaylibThread) {
        self.background_texture = rl.load_texture_from_bytes(rt, ".png", include_bytes!("..\\resources\\background.png"));
        self.blocks_texture = rl.load_texture_from_bytes(rt, ".png", include_bytes!("..\\resources\\blocks.png"));
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        self.game.input.set(InputType::MoveLeft, rl.is_key_down(KeyboardKey::KEY_LEFT));
        self.game.input.set(InputType::MoveRight, rl.is_key_down(KeyboardKey::KEY_RIGHT));
        self.game.input.set(InputType::SoftDrop, rl.is_key_down(KeyboardKey::KEY_DOWN));
        self.game.input.set(InputType::HardDrop, rl.is_key_down(KeyboardKey::KEY_SPACE));
        self.game.input.set(InputType::RotateCW, rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_X));
        self.game.input.set(InputType::RotateCCW, rl.is_key_down(KeyboardKey::KEY_Z));
        self.game.input.set(InputType::Flip, rl.is_key_down(KeyboardKey::KEY_A));
        self.game.input.set(InputType::Hold, rl.is_key_down(KeyboardKey::KEY_C));

        // for Debug
        let mouse_left = rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);
        let mouse_right = rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON);

        if mouse_left || mouse_right {
            let mouse_position = rl.get_mouse_position();
            let board_width = self.game.board.width() as i32;
            let board_height = self.game.board.height() as i32;
            let board_x = (self.screen_width - CELL_SIZE * board_width) / 2;
            let board_y = (self.screen_height + CELL_SIZE * board_height) / 2;
            let grid_x = (mouse_position.x as i32 - board_x) / CELL_SIZE;
            let grid_y = (board_y - mouse_position.y as i32) / CELL_SIZE;

            if mouse_left {
                self.game.board.set_block(grid_x, grid_y, BlockType::Green);
            } else if mouse_right {
                self.game.board.set_block(grid_x, grid_y, BlockType::Empty);
            }
        }

        self.game.update(rl.get_frame_time());
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);

        if let Some(texture) = &self.background_texture {
            d.draw_texture(texture, 0, 0, Color::WHITE);
        }

        let board_width = self.game.board.width() as i32;
        let board_height = self.game.board.height() as i32;
        let draw_left = (self.screen_width - CELL_SIZE * board_width) / 2;
        let draw_right = draw_left + CELL_SIZE * board_width;
        let draw_bottom = (self.screen_height + CELL_SIZE * board_height) / 2;
        let draw_top = draw_bottom - CELL_SIZE * board_height;

        // Board
        self.draw_panel(d, draw_left, draw_top, board_width * CELL_SIZE, board_height * CELL_SIZE);

        for y in 0..self.game.board.row_count() as i32 {
            for x in 0..board_width {
                self.draw_block(d, draw_left, draw_bottom, x, y, self.game.board.get_block(x, y), 255);
            }
        }

        if let Some(piece) = &self.game.current_piece {
            let mut ghost = piece.clone();

            while ghost.shift(&self.game.board, 0, -1) {
            }

            for y in 0..4 {
                for x in 0..4 {
                    self.draw_block(d, draw_left, draw_bottom, x + ghost.x, y + ghost.y, ghost.get_block(x, y), 80);
                }
            }

            for y in 0..4 {
                for x in 0..4 {
                    self.draw_block(d, draw_left, draw_bottom, x + piece.x, y + piece.y, piece.get_block(x, y), 255);
                }
            }
        }

        // Hold
        d.draw_text_right("Hold", draw_left - 20, draw_top, 20, Color::WHITE);
        self.draw_panel(d, draw_left - 121, draw_top + 29, 102, 82);

        if let Some(hold_piece) = self.game.hold_piece {
            self.draw_preview(d, draw_left - 70, draw_top + 70, hold_piece);
        }

        // Next
        d.draw_text("Next", draw_right + 20, draw_top, 20, Color::WHITE);
        self.draw_panel(d, draw_right + 19, draw_top + 29, 102, 322);

        for i in 0..5 {
            self.draw_preview(d, draw_right + 70, draw_top + 70 + i * 60, self.game.bag.get(i));
        }

        // Level
        d.draw_text_right("Level", draw_left - 20, draw_top + 140, 20, Color::WHITE);
        d.draw_text_right(format!("{}", self.game.level).as_str(), draw_left - 20, draw_top + 166, 40, Color::WHITE);

        // Lines
        d.draw_text_right("Lines", draw_left - 20, draw_top + 222, 20, Color::WHITE);
        d.draw_text_right(format!("{}", self.game.lines).as_str(), draw_left - 20, draw_top + 246, 40, Color::WHITE);

        // Score
        d.draw_text_right("Score", draw_left - 20, draw_top + 300, 20, Color::WHITE);
        d.draw_text_right(format!("{}", self.game.score).as_str(), draw_left - 20, draw_top + 326, 40, Color::WHITE);
    }

    fn draw_block(&self, d: &mut RaylibDrawHandle, left: i32, bottom: i32, x: i32, y: i32, block_type: BlockType, alpha: u8) {
        match block_type {
            BlockType::Empty | BlockType::Outside => { },
            _ => {
                if let Some(texture) = &self.blocks_texture {
                    d.draw_texture_pro(texture,
                        rrect(block_type as i32 * 30, 0, 30, 30),
                        rrect(left + x * CELL_SIZE, bottom - (y + 1) * CELL_SIZE, CELL_SIZE, CELL_SIZE),
                        rvec2(0, 0),
                        0.0,
                        Color::new(255, 255, 255, alpha)
                    );
                }
            }
        }
    }

    fn draw_preview(&self, d: &mut RaylibDrawHandle, x: i32, y: i32, piece_type: PieceType) {
        if let Some(texture) = &self.blocks_texture {
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
                d.draw_texture_pro(texture,
                    rrect(piece_type.get_block_type() as i32 * 30, 0, 30, 30),
                    rrect(x + ((cell.0 - 0.5) * PREVIEW_CELL_SIZE as f32) as i32, y + ((cell.1 - 0.5) * PREVIEW_CELL_SIZE as f32) as i32, PREVIEW_CELL_SIZE, PREVIEW_CELL_SIZE),
                    rvec2(0, 0),
                    0.0,
                    Color::WHITE
                );
            }
        }
    }

    fn draw_panel(&self, d: &mut RaylibDrawHandle, x: i32, y: i32, width: i32, height: i32) {
        d.draw_rectangle_lines_ex(Rectangle {
            x: (x - 2) as f32,
            y: (y - 2) as f32,
            width: (width + 4) as f32,
            height: (height + 4) as f32
        }, 2, Color::WHITE);

        d.draw_rectangle(x, y, width, height, Color { r: 0, g: 0, b: 0, a: 224 });

    }
}