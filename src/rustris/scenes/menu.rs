use macroquad::prelude::*;
use macroquad::color::colors;
use super::super::*;

pub struct Menu {
}

impl Menu {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::Enter) {
            set_scene(Scene::Ingame);
        }
    }

    pub fn draw(&self) {
        let scale = screen_height() / 720.0;

        push_matrix_trs(screen_width() * 0.5, screen_height() * 0.5, 0.0, scale, scale);

        draw_text_aligned("Press Enter to Start", 0.0, 0.0, *DEFAULT_FONT, 30, 0.5, 0.5, colors::WHITE);

        pop_matrix();
    }
}