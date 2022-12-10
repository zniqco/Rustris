mod core;

use raylib::prelude::*;
use self::core::*;

pub struct Rustris {
    screen_width: i32,
    screen_height: i32,
    game: Core,
}

const CELL_SIZE: i32 = 30;

impl Rustris {
    pub fn new(screen_width: i32, screen_height: i32) -> Self {
        Self {
            screen_width,
            screen_height,
            game: Core::new(),
        }
    }

    pub fn init(&mut self) {
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

        let board_width = self.game.board.width() as i32;
        let board_height = self.game.board.height() as i32;
        let draw_left = (self.screen_width - CELL_SIZE * board_width) / 2;
        let draw_bottom = (self.screen_height + CELL_SIZE * board_height) / 2;
        let draw_top = draw_bottom - CELL_SIZE * board_height;

        // Border
        d.draw_rectangle_lines(draw_left - 1, draw_top - 1, board_width * CELL_SIZE + 2, board_height * CELL_SIZE + 2, Color::WHITE);

        // Block
        for y in 0..board_height {
            for x in 0..board_width {
                self.draw_block(d, draw_left, draw_bottom, x, y, self.game.board.get_block(x, y), 255);
            }
        }

        if let Some(piece) = &self.game.current_piece {
            // Ghost
            let mut ghost = piece.clone();

            while ghost.shift(&self.game.board, 0, -1) {
            }

            for y in 0..4 {
                for x in 0..4 {
                    self.draw_block(d, draw_left, draw_bottom, x + ghost.x, y + ghost.y, ghost.get_block(x, y), 80);
                }
            }

            // Piece
            for y in 0..4 {
                for x in 0..4 {
                    self.draw_block(d, draw_left, draw_bottom, x + piece.x, y + piece.y, piece.get_block(x, y), 255);
                }
            }
        }

        d.draw_text("Score", draw_left + CELL_SIZE * board_width + 10, draw_top, 20, Color::WHITE);
        d.draw_text(format!("{}", self.game.score).as_str(), draw_left + CELL_SIZE * board_width + 10, draw_top + 24, 30, Color::WHITE);
    }

    fn draw_block(&self, d: &mut RaylibDrawHandle, left: i32, bottom: i32, x: i32, y: i32, block_type: BlockType, alpha: u8) {
        match block_type {
            BlockType::Empty | BlockType::Outside => { },
            _ => d.draw_rectangle(left + x * CELL_SIZE, bottom - (y + 1) * CELL_SIZE, CELL_SIZE, CELL_SIZE, block_type.get_color(alpha)),
        }
    }
}