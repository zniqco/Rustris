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
    fn update(&mut self) -> Vec<ObjectEvent> {
        let mut events = vec![];

        if is_key_down(KeyCode::Enter) {
            events.push(ObjectEvent::Create { 
                depth: 0,
                object: Ingame::new().into()
            });
            
            events.push(ObjectEvent::Destroy);
        }

        events
    }

    fn draw(&self) {
        draw_text_aligned("Press Enter to Start", 0.0, 0.0, *DEFAULT_FONT, 30, 0.5, 0.5, Color::from_rgba(255, 255, 255, 255));
    }
}