use macroquad::prelude::*;
use super::*;

pub struct PlaceEffect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    alpha: f32,
}

impl PlaceEffect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            alpha: 1.0,
        }
    }
}

impl Object for PlaceEffect {
    fn should_destroy(&self) -> bool {
        self.alpha <= 0.0
    }

    fn depth(&self) -> i32 {
        -1
    }

    fn update(&mut self) {
        self.alpha -= get_frame_time() / 0.05;
    }

    fn draw(&self) {
        gl_use_material(*ADDITIVE_MATERIAL);
        draw_rectangle(self.x, self.y, self.width, self.height, Color::new(1.0, 1.0, 1.0, self.alpha * 0.3));
        gl_use_default_material();
    }
}
