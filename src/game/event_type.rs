use super::*;

pub enum EventType {
    LockReset,
    Pointed { score: i32, lines: i32, combo: i32, tspin: TSpinType },
}
