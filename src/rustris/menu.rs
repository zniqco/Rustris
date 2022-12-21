use macroquad::prelude::*;
use super::*;

pub struct Menu {
    is_destroyed: bool,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            is_destroyed: false,
        }
    }
}

impl Object for Menu {
    fn should_destroy(&self) -> bool {
        self.is_destroyed
    }

    fn update(&mut self) {
        if !self.is_destroyed {
            if is_key_down(KeyCode::Enter) {
                object_add(ObjectType::Ingame(Ingame::new()));
                
                self.is_destroyed = true;
            }
        }
    }

    fn draw(&self) {
        draw_text_aligned("Press Enter to Start", 0.0, 0.0, *DEFAULT_FONT, 30, 0.5, 0.5, Color::from_rgba(255, 255, 255, 255));
    }
}