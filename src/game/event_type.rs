use super::*;

pub enum EventType {
    LockReset,
    Pointed { score: i32, lines: i32, combo: i32, b2b: bool, tspin: TSpinType },
    Placed { positions: Vec<(i32, i32)> },
}
