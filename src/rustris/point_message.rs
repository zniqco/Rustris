use macroquad::prelude::*;
use super::*;

pub struct PointMessage {
    x: f32,
    y: f32,
    delta: f32,
    message: String,
    sub_message: String,
}

impl PointMessage {
    pub fn new(x: f32, y: f32, message: String, sub_message: String) -> Self {
        Self {
            x,
            y,
            delta: 1.0,
            message,
            sub_message,
        }
    }
}

impl Object for PointMessage {
    fn update(&mut self) {
        let dt = get_frame_time();

        self.delta -= dt / 0.75;
        self.y -= dt * 24.0;

        if self.delta <= 0.0 {
            object_destroy(self);
        }
    }

    fn draw(&self) {
        let scale = 1.0 - simple_easing::elastic_in(inverse_lerp(0.3, 1.0, self.delta));
        let draw_x = self.x;
        let draw_y = self.y;
        let alpha = (inverse_lerp(0.0, 0.3, self.delta) * 255.0) as u8;

        push_matrix_trs(draw_x, draw_y, 0.0, scale, scale);

        if !self.sub_message.is_empty() {
            draw_text_aligned(self.message.as_str(), 0.0, 2.0, font_default(), 38, 0.5, 1.0, Color::from_rgba(255, 255, 255, alpha)); 
            draw_text_aligned(self.sub_message.as_str(), 0.0, 6.0, font_default(), 22, 0.5, 0.0, Color::from_rgba(255, 255, 255, alpha)); 
        } else {
            draw_text_aligned(self.message.as_str(), 0.0, 0.0, font_default(), 38, 0.5, 0.5, Color::from_rgba(255, 255, 255, alpha));     
        }

        pop_matrix();
    }
}
