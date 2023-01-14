mod object;
mod background;
mod functions;
mod board;
mod menu;
mod point_message;

use object::*;
use background::*;
use functions::*;
use board::*;
use menu::*;
use point_message::*;

use macroquad::prelude::*;

pub struct Rustris {
}

impl Rustris {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn init(&mut self) {
        object_add(10000, Background::new());
        object_add(0, Menu::new());
    }

    pub fn update(&mut self) {
        object_update();
    }

    pub fn draw(&self) {
        clear_background(Color::new(0.0, 0.0, 0.0, 1.0));
    
        object_draw();

        draw_text_aligned(format!("{:02.1}", 1.0 / get_frame_time()).as_str(), screen_width() - 12.0, 16.0, font_default(), 22, 1.0, 0.0, Color::new(1.0, 1.0, 1.0, 0.75));
        draw_text_aligned(object_count().to_string().as_str(), screen_width() - 12.0, 36.0, font_default(), 22, 1.0, 0.0, Color::new(1.0, 1.0, 1.0, 0.75));
    }

    pub fn quitted(&self) -> bool {
        quitted()
    }
}
