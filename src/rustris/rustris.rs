use std::sync::Mutex;
use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use macroquad::prelude::*;
use super::*;

lazy_static! {
    pub static ref OBJECTS: Mutex<Vec<ObjectType>> = Mutex::new(vec![]);
    static ref RESERVED_OBJECTS: Mutex<Vec<ObjectType>> = Mutex::new(vec![]);
}

#[enum_dispatch]
pub trait Object {
    fn should_destroy(&self) -> bool { false }
    fn depth(&self) -> i32 { 0 }
    fn update(&mut self) { }
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

pub fn init() {
    init_resources();

    object_add(ObjectType::Background(Background::new()));
    object_add(ObjectType::Menu(Menu::new()));
}

pub fn update() {
    object_update();
}

pub fn draw() {
    clear_background(Color::from_rgba(0, 0, 0, 255));

    let scale = screen_height() / 720.0;

    push_matrix_trs(screen_width() * 0.5, screen_height() * 0.5, 0.0, scale, scale);

    object_draw();

    pop_matrix();

    draw_text_aligned(get_fps().to_string().as_str(), screen_width() - 12.0, 16.0, *DEFAULT_FONT, 18, 1.0, 0.0, Color::from_rgba(255, 255, 255, 255));
    draw_text_aligned(OBJECTS.lock().unwrap().len().to_string().as_str(), screen_width() - 12.0, 36.0, *DEFAULT_FONT, 12, 1.0, 0.0, Color::from_rgba(255, 255, 255, 255));
}

pub fn object_add(object: ObjectType) {
    RESERVED_OBJECTS.lock().unwrap().push(object);
}

pub fn object_update() {
    let mut objects = OBJECTS.lock().unwrap();

    objects.sort_by(|a, b| b.depth().cmp(&a.depth()));

    for object in objects.iter_mut() {
        object.update();
    }

    // Add
    let mut reserved_objects = RESERVED_OBJECTS.lock().unwrap();

    loop {
        match reserved_objects.pop() {
            Some(x) => objects.push(x),
            None => break,
        }
    }

    drop(reserved_objects);

    // Destroy
    objects.retain(|x| !x.should_destroy());
}

pub fn object_draw() {
    for object in OBJECTS.lock().unwrap().iter() {
        object.draw();
    }
}
