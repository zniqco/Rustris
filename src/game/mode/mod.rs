mod marathon;
mod zen;

use super::*;

#[derive(Clone, Copy)]
pub struct Mode {
    pub das: Option<f32>,
    pub arr: Option<f32>,
    pub sdf: Option<f32>,
    pub width: usize,
    pub height: usize,
    pub randomizer: RandomizerType,
    pub rotation: Option<RotationType>,
    pub levels: &'static [LevelData],
}

#[derive(Clone, Copy)]
pub struct LevelData {
    pub gravity: f32,
    pub lock_delay: f32,
    pub lines: i32, // 0 = Infinity
}

#[derive(Clone, Copy)]
pub enum ModeType {
    Marathon,
    Zen,
}

impl ModeType {
    pub fn to_struct(&self) -> Mode {
        match self {
            ModeType::Marathon => marathon::MODE_MARATHON,
            ModeType::Zen => zen::MODE_ZEN,
        }
    }
}
