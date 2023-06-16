use std::default::Default;
use super::*;

#[derive(Clone, Copy)]
pub struct Config {
    pub das: f32,
    pub arr: f32,
    pub sdf: f32,
    pub rotation: RotationType,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            das: 10.0 / 60.0,
            arr: 1.0 / 60.0,
            sdf: 1.0 / 60.0,
            rotation: RotationType::SRS,
        }
    }
}
