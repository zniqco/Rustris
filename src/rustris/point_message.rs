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
    fn should_destroy(&self) -> bool {
        self.delta <= 0.0
    }

    fn depth(&self) -> i32 {
        -1
    }

    fn update(&mut self) {
        self.delta -= get_frame_time();
    }

    fn draw(&self) {
        let scale = 1.0 + simple_easing::expo_in(self.delta) * 0.4;
        let draw_x = self.x;
        let draw_y = self.y;
        let alpha = (self.delta * 3.0).min(1.0);

        push_matrix_trs(draw_x, draw_y, 0.0, scale, scale);

        if self.sub_message.len() >= 1 {
            draw_text_aligned(self.sub_message.as_str(), 0.0, 0.0, *DEFAULT_FONT, 18, 0.0, 1.0, Color::from_rgba(255, 255, 255, (alpha * 255.0) as u8)); 
            draw_text_aligned(self.message.as_str(), 0.0, -18.0, *DEFAULT_FONT, 32, 0.0, 1.0, Color::from_rgba(255, 255, 255, (alpha * 255.0) as u8)); 
        } else {
            draw_text_aligned(self.message.as_str(), 0.0, 0.0, *DEFAULT_FONT, 32, 0.0, 1.0, Color::from_rgba(255, 255, 255, (alpha * 255.0) as u8));     
        }

        pop_matrix();
    }
}
