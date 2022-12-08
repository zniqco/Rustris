mod core;

use raylib::prelude::*;
use self::core::*;

pub struct Rustris {
    game: Core
}

const DRAW_X: i32 = (640 - 160) / 2;
const DRAW_Y: i32 = (480 - 320) / 2;

impl Rustris {
    pub fn new() -> Self {
        Self {
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

        self.game.update(rl.get_frame_time());
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);

        d.draw_rectangle(DRAW_X, DRAW_Y, 10 * 16, 20 * 16, Color::DARKGRAY);

        for y in 0..20 {
            for x in 0..self.game.board.get_width() {
                let block = self.game.board.get_block(x, y);

                self.draw_block(d, x, y, block);
            }
        }

        if let Some(piece) = &self.game.current_piece {
            for y in 0..4 {
                for x in 0..4 {
                    let block = piece.get_block(x, y);

                    self.draw_block(d, x + piece.x, y + piece.y, block);
                }
            }
        }

        d.draw_text("Score", DRAW_X + 180, DRAW_Y, 10, Color::WHITE);
        d.draw_text(format!("{}", self.game.score).as_str(), DRAW_X + 180, DRAW_Y + 16, 20, Color::WHITE);
    }

    fn draw_block(&self, d: &mut RaylibDrawHandle, x: i32, y: i32, block: BlockType) {
        let draw_x = DRAW_X + x * 16;
        let draw_y = DRAW_Y + (19 - y) * 16;

        match block {
            BlockType::Red => d.draw_rectangle(draw_x, draw_y, 16, 16, Color::RED),
            BlockType::Orange => d.draw_rectangle(draw_x, draw_y, 16, 16, Color::ORANGE),
            BlockType::Yellow => d.draw_rectangle(draw_x, draw_y, 16, 16, Color::YELLOW),
            BlockType::Green => d.draw_rectangle(draw_x, draw_y, 16, 16, Color::GREEN),
            BlockType::Blue => d.draw_rectangle(draw_x, draw_y, 16, 16, Color::BLUE),
            BlockType::Cyan => d.draw_rectangle(draw_x, draw_y, 16, 16, Color::SKYBLUE),
            BlockType::Purple => d.draw_rectangle(draw_x, draw_y, 16, 16, Color::PURPLE),
            _ => { }
        }
    }
}