use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum BlockType {
    Empty,
    Outside,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Cyan,
    Purple,
}

impl BlockType {
    pub fn get_color(&self, a: u8) -> Color {
        match self {
            BlockType::Red => Color { r: 255, g: 0, b: 0, a },
            BlockType::Orange => Color { r: 255, g: 128, b: 0, a },
            BlockType::Yellow => Color { r: 255, g: 255, b: 0, a },
            BlockType::Green => Color { r: 0, g: 255, b: 0, a },
            BlockType::Blue => Color { r: 0, g: 0, b: 255, a },
            BlockType::Cyan => Color { r: 0, g: 255, b: 255, a },
            BlockType::Purple => Color { r: 160, g: 0, b: 255, a },
            _ => Color::BLANK,
        }
    }
}
