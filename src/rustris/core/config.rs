use std::default::Default;

pub struct Config {
    pub das: f32,
    pub arr: f32,
    pub sdf: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            das: 10.0 / 60.0,
            arr: 1.0 / 60.0,
            sdf: 1.0 / 60.0,
        }
    }
}
