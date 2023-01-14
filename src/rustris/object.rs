use std::{sync::{Mutex, RwLock}, collections::{HashMap, HashSet, VecDeque}};
use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use super::*;

lazy_static! {
    static ref OBJECTS: RwLock<HashMap<u64, ObjectData>> = RwLock::new(HashMap::new());
    static ref CREATE_RESERVED: Mutex<VecDeque<ObjectData>> = Mutex::new(VecDeque::new());
    static ref DESTROY_RESERVED: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());
    static ref LAST_ID: Mutex<u64> = Mutex::new(0);
}

#[enum_dispatch]
pub trait Object {
    fn init(&mut self) { }
    fn update(&mut self) -> Vec<ObjectEvent> { Vec::new() }
    fn draw(&self);
}

#[derive(PartialEq, Eq)]
#[enum_dispatch(Object)]
pub enum ObjectType {
    Background,
    Menu,
    Board,
    PointMessage,
}

pub enum ObjectEvent {
    DestroySelf,
}

struct ObjectData {
    pub id: u64,
    pub depth: i32,
    pub object: ObjectType,
}

pub fn object_add<T: Into<ObjectType>>(depth: i32, object: T) -> u64 {
    let mut last_id_lock = LAST_ID.lock().unwrap();
    let id = *last_id_lock;

    *last_id_lock = *last_id_lock + 1;

    CREATE_RESERVED.lock().unwrap().push_back(ObjectData {
        id,
        depth,
        object: object.into(),
    });

    id
}

pub fn object_destroy(id: u64) {
    DESTROY_RESERVED.lock().unwrap().insert(id);
}

pub fn object_fetch(id: u64) {
}

pub fn object_update() {
    let mut objects = OBJECTS.write().unwrap();

    for data in objects.values_mut() {
        let events = data.object.update();

        for event in events {
            match event {
                ObjectEvent::DestroySelf => {
                    object_destroy(data.id);
                },
            }
        }
    }

    // Add
    let mut create = CREATE_RESERVED.lock().unwrap();

    while let Some(mut x) = create.pop_front() {
        x.object.init();
        objects.insert(x.id, x);
    }

    drop(create);

    // Destroy
    let mut destroy = DESTROY_RESERVED.lock().unwrap();

    for id in destroy.iter() {
        objects.remove(id);
    }

    destroy.clear();
}

pub fn object_draw() {
    let scale = screen_height() / 720.0;
    let objects = OBJECTS.read().unwrap();
    let mut sorted = Vec::from_iter(objects.values());
    
    sorted.sort_by(|a, b| b.depth.cmp(&a.depth));

    push_matrix_trs(screen_width() * 0.5, screen_height() * 0.5, 0.0, scale, scale);

    for data in sorted {
        data.object.draw();
    }

    pop_matrix();
}

pub fn object_count() -> usize {
    OBJECTS.read().unwrap().len()
}