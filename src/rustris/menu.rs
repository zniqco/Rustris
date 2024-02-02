use macroquad::prelude::*;
use macroquad::audio::play_sound_once;
use crate::game::{Config, ModeType};

use super::*;

enum MenuItem {
    Default { caption: &'static str, callback: fn(&mut Menu) -> () },
}

pub struct Menu {
    item_stack: Vec<Vec<MenuItem>>,
    index_stack: Vec<usize>,
    current_index: usize,
    shake_delta: f32,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            item_stack: Vec::new(),
            index_stack: Vec::new(),
            current_index: 0,
            shake_delta: 0.0,
        }
    }
}

impl Menu {
    fn enqueue(&mut self, items: Vec<MenuItem>) {
        self.item_stack.insert(0, items);
        self.index_stack.insert(0, self.current_index);

        self.current_index = 0;
        self.shake_delta = 0.0;
    }

    fn dequeue(&mut self) {
        self.current_index = self.index_stack[0];
        self.shake_delta = 0.0;

        self.item_stack.remove(0);
        self.index_stack.remove(0);
    }

    fn menu_main() -> Vec<MenuItem> {
        vec![
            MenuItem::Default {
                caption: "START",
                callback: |menu| {
                    menu.enqueue(Self::menu_start());
                }
            },
            MenuItem::Default {
                caption: "CONFIG",
                callback: |menu| {
                    menu.enqueue(Self::menu_config());
                }
            },
            MenuItem::Default {
                caption: "EXIT",
                callback: |_menu| {
                    quit();
                }
            },
        ]
    }

    fn menu_start() -> Vec<MenuItem> {
        vec![
            MenuItem::Default {
                caption: "MARATHON",
                callback: |menu| {
                    object_destroy(menu);
                    object_add(0, Board::new(Config::default(), ModeType::Marathon));
                }
            },
            MenuItem::Default {
                caption: "ZEN",
                callback: |menu| {
                    object_destroy(menu);
                    object_add(0, Board::new(Config::default(), ModeType::Zen));
                }
            },
            MenuItem::Default {
                caption: "BACK",
                callback: |menu| {
                    menu.dequeue();
                }
            },
        ]
    }

    fn menu_config() -> Vec<MenuItem> {
        vec![
            MenuItem::Default {
                caption: "(UPDATE SOON)",
                callback: |_menu| {
                }
            },
            MenuItem::Default {
                caption: "BACK",
                callback: |menu| {
                    menu.dequeue();
                }
            },
        ]
    }
}

impl Object for Menu {
    fn init(&mut self) {
        self.item_stack.push(Self::menu_main());
    }

    fn update(&mut self) {
        let dt = get_frame_time();

        if self.shake_delta.abs() <= dt / 0.5 {
            self.shake_delta = 0.0;
        } else {
            self.shake_delta -= dt / 0.5 * self.shake_delta.signum();
        }

        if let Some(items) = self.item_stack.first_mut() {
            if is_key_pressed(KeyCode::Up) && self.current_index > 0 {
                self.current_index -= 1;
                self.shake_delta = -1.0;

                play_sound_once(sound("move"));
            }

            if is_key_pressed(KeyCode::Down) && self.current_index < items.len() - 1 {
                self.current_index += 1;
                self.shake_delta = 1.0;

                play_sound_once(sound("move"));
            }

            if is_key_pressed(KeyCode::Enter) {
                play_sound_once(sound("lock"));
                
                match items[self.current_index] {
                    MenuItem::Default { caption: _, callback } => {
                        callback(self);
                    }
                }
            }
        }
    }

    fn draw(&self) {
        if let Some(items) = self.item_stack.first() {
            const ITEM_SIZE: f32 = 60.0;
            let count = items.len();
            let mut draw_y = (count - 1) as f32 * -ITEM_SIZE * 0.5;

            for i in 0..count {
                let color = match self.current_index == i {
                    true => Color::new(1.0, 1.0, 1.0, 1.0),
                    false => Color::new(0.75, 0.75, 0.75, 1.0),
                };

                let offset = match self.current_index == i {
                    true => simple_easing::elastic_in(self.shake_delta.abs()) * self.shake_delta.signum() * 10.0,
                    false => 0.0,
                };

                match items[i] {
                    MenuItem::Default { caption, callback: _ } => {
                        draw_text_aligned(caption, 0.0, draw_y + offset, &FONT_DEFAULT, 56, 0.5, 0.5, color);
                    },
                }

                draw_y += ITEM_SIZE;
            }
        }
    }
}
