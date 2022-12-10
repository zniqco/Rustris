use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum BlockType {
    Empty = -2,
    Outside = -1,
    Red = 0,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
    Gray,
}
