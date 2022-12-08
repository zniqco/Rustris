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
            let board_y = (self.screen_height - CELL_SIZE * board_height) / 2;
            let grid_x = (mouse_position.x as i32 - board_x) / CELL_SIZE;
            let grid_y = (board_y + CELL_SIZE * (board_height + 2) - mouse_position.y as i32 - board_y) / CELL_SIZE;

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
        let board_x = (self.screen_width - CELL_SIZE * board_width) / 2;
        let board_y = (self.screen_height - CELL_SIZE * board_height) / 2;

        d.draw_rectangle_lines(board_x - 1, board_y - 1, 10 * CELL_SIZE + 2, 20 * CELL_SIZE + 2, Color::WHITE);

        for y in 0..board_height {
            for x in 0..board_width {
                let block = self.game.board.get_block(x, y);

                self.draw_block(d, board_x, board_y, x, y, block);
            }
        }

        if let Some(piece) = &self.game.current_piece {
            for y in 0..4 {
                for x in 0..4 {
                    let block = piece.get_block(x, y);

                    self.draw_block(d, board_x, board_y, x + piece.x, y + piece.y, block);
                }
            }
        }

        d.draw_text("Score", board_x + CELL_SIZE * board_width + 10, board_y, 20, Color::WHITE);
        d.draw_text(format!("{}", self.game.score).as_str(), board_x + CELL_SIZE * board_width + 10, board_y + 24, 30, Color::WHITE);
    }

    fn draw_block(&self, d: &mut RaylibDrawHandle, board_x: i32, board_y: i32, x: i32, y: i32, block: BlockType) {
        let draw_x = board_x + x * CELL_SIZE;
        let draw_y = board_y + (self.game.board.height() as i32 - 1 - y) * CELL_SIZE;

        match block {
            BlockType::Empty | BlockType::Outside => { },
            _ => d.draw_rectangle(draw_x, draw_y, CELL_SIZE, CELL_SIZE, block.get_color()),
        }
    }
}