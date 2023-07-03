use std::{collections::{HashMap, HashSet, VecDeque}, ptr};
use once_cell::sync::Lazy;
use enum_dispatch::enum_dispatch;
use super::*;

static mut OBJECTS: Lazy<HashMap<u64, ObjectData>> = Lazy::new(HashMap::new);
static mut CREATE_RESERVED: Lazy<VecDeque<ObjectData>> = Lazy::new(VecDeque::new);
static mut DESTROY_RESERVED: Lazy<HashSet<u64>> = Lazy::new(HashSet::new);
static mut LAST_ID: Lazy<u64> = Lazy::new(|| 0);

#[enum_dispatch]
pub trait Object {
    fn init(&mut self) { }
    fn update(&mut self) { }
    fn draw(&self);
    fn ptr(&self) -> *const () { ptr::addr_of!(*self) as *const () }
}

#[enum_dispatch(Object)]
pub enum ObjectType {
    Background,
    Menu,
    Board,
    PointMessage,
}

struct ObjectData {
    pub id: u64,
    pub depth: i32,
    pub object: ObjectType,
}

pub fn object_add<T: Into<ObjectType>>(depth: i32, object: T) -> u64 {
    unsafe {
        let id = *LAST_ID;

        *LAST_ID += 1;

        CREATE_RESERVED.push_back(ObjectData {
            id,
            depth,
            object: object.into(),
        });

        id
    }
}

pub fn object_destroy<T: Object>(object: &T) {
    unsafe {
        let this = ptr::addr_of!(*object) as *const ();

        for data in OBJECTS.values_mut() {
            if this == data.object.ptr() {
                object_destroy_by_id(data.id);

                return;
            }
        }
    }

    panic!()
}

pub fn object_destroy_by_id(id: u64) {
    unsafe {
        DESTROY_RESERVED.insert(id);
    }
}

pub fn object_update() {
    unsafe {
        for data in OBJECTS.values_mut() {
            data.object.update();
        }

        // Add
        while let Some(mut x) = CREATE_RESERVED.pop_front() {
            x.object.init();
            OBJECTS.insert(x.id, x);
        }

        // Destroy
        for id in DESTROY_RESERVED.iter() {
            OBJECTS.remove(id);
        }

        DESTROY_RESERVED.clear();
    }
}

pub fn object_draw() {
    let scale = screen_height() / 720.0;

    unsafe {
        let mut sorted = Vec::from_iter(OBJECTS.values());
        
        sorted.sort_by(|a, b| b.depth.cmp(&a.depth));

        push_matrix_trs(screen_width() * 0.5, screen_height() * 0.5, 0.0, scale, scale);

        for data in sorted {
            data.object.draw();
        }

        pop_matrix();
    }
}

pub fn object_count() -> usize {
    unsafe {
        OBJECTS.len()
    }
}
