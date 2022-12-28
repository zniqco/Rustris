mod background;
mod functions;
mod ingame;
mod menu;
mod place_effect;
mod point_message;
mod resources;

use background::*;
use functions::*;
use ingame::*;
use menu::*;
use place_effect::*;
use point_message::*;
use resources::*;

use enum_dispatch::enum_dispatch;
use macroquad::prelude::*;

#[enum_dispatch]
pub trait Object {
    fn update(&mut self) -> Vec<ObjectEvent> { Vec::new() }
    fn draw(&self);
}

#[enum_dispatch(Object)]
pub enum ObjectType {
    Background,
    Menu,
    Ingame,
    PlaceEffect,
    PointMessage,
}

pub enum ObjectEvent {
    Create { depth: i32, object: ObjectType },
    Destroy,
}

struct ObjectData {
    pub id: u64,
    pub depth: i32,
    pub object: ObjectType,
    pub should_destroy: bool,
}

pub struct Rustris {
    objects: Vec<ObjectData>,
    last_id: u64,
}

impl Rustris {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            last_id: 0,
        }
    }

    pub fn init(&mut self) {
        Self::add_object(&mut self.last_id, &mut self.objects, 10000, Background::new().into());
        Self::add_object(&mut self.last_id, &mut self.objects, 0, Menu::new().into());
    }

    pub fn update(&mut self) {
        let objects = &mut self.objects;
        let mut reserved_objects: Vec<ObjectData> = Vec::new();

        for pair in objects.iter_mut() {
            let events = pair.object.update();

            for event in events {
                match event {
                    ObjectEvent::Create { depth, object } => {
                        Self::add_object(&mut self.last_id, &mut reserved_objects, depth, object);
                    },
                    ObjectEvent::Destroy => {
                        pair.should_destroy = true;
                    },
                }
            }
        }

        objects.retain(|data| !data.should_destroy);

        while let Some(x) = reserved_objects.pop() {
            objects.push(x);
        }

        objects.sort_by(|a, b| b.depth.cmp(&a.depth));
    }

    pub fn draw(&self) {
        clear_background(Color::from_rgba(0, 0, 0, 255));
    
        let scale = screen_height() / 720.0;
    
        push_matrix_trs(screen_width() * 0.5, screen_height() * 0.5, 0.0, scale, scale);

        for object in self.objects.iter() {
            object.object.draw();
        }

        pop_matrix(); 
    
        draw_text_aligned(get_fps().to_string().as_str(), screen_width() - 12.0, 16.0, *DEFAULT_FONT, 18, 1.0, 0.0, Color::from_rgba(255, 255, 255, 255));
        draw_text_aligned(self.objects.len().to_string().as_str(), screen_width() - 12.0, 36.0, *DEFAULT_FONT, 12, 1.0, 0.0, Color::from_rgba(255, 255, 255, 255));
    }

    fn add_object(last_id: &mut u64, objects: &mut Vec<ObjectData>, depth: i32, object: ObjectType) {
        let id = *last_id + 1;

        *last_id = id;

        objects.push(ObjectData {
            id,
            depth,
            object: object.into(),
            should_destroy: false,
        });
    }
}
