use macroquad::prelude::*;
use super::*;

pub struct Background {
}

impl Background {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Object for Background {
    fn depth(&self) -> i32 {
        10000
    }

    fn draw(&self) {
        let width = screen_width() as f32;
        let height = 720.0;

        draw_texture_ex(*BACKGROUND_TEXTURE, width * -0.5, height * -0.5, Color::from_rgba(255, 255, 255, 255), DrawTextureParams {
            dest_size: Some(Vec2::new(width, height)),
            ..Default::default()
        });
    }
}
