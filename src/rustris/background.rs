use macroquad::prelude::*;
use super::*;

pub struct Background {
    alpha: f32,
}

impl Background {
    pub fn new() -> Self {
        Self {
            alpha: 0.0,
        }
    }
}

impl Object for Background {
    fn update(&mut self) -> Vec<ObjectEvent> {
        let dt = get_frame_time();

        self.alpha = (self.alpha + dt / 0.5).clamp(0.0, 1.0);

        Vec::new()
    }
    
    fn draw(&self) {
        let width = screen_width();
        let height = 720.0;

        draw_texture_ex(texture("background_1"), width * -0.5, height * -0.5, Color::from_rgba(255, 255, 255, (self.alpha * 128.0) as u8), DrawTextureParams {
            dest_size: Some(Vec2::new(width, height)),
            ..Default::default()
        });
    }
}
