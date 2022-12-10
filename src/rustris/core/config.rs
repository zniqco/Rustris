use std::default::Default;
use super::*;

pub struct Config {
    pub das: f32,
    pub arr: f32,
    pub sdf: f32,
    pub width: usize,
    pub height: usize,
    pub levels: Vec<LevelData>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            das: 10.0 / 60.0,
            arr: 1.0 / 60.0,
            sdf: 1.0 / 60.0,
            width: 10,
            height: 20,
            levels: vec![
                LevelData { gravity: 0.01667, lock_delay: 1000.0, lines: 10 },
                LevelData { gravity: 0.021017, lock_delay: 970.0, lines: 10 },
                LevelData { gravity: 0.026977, lock_delay: 940.0, lines: 10 },
                LevelData { gravity: 0.035256, lock_delay: 910.0, lines: 10 },
                LevelData { gravity: 0.04693, lock_delay: 880.0, lines: 10 },
                LevelData { gravity: 0.06361, lock_delay: 850.0, lines: 10 },
                LevelData { gravity: 0.0879, lock_delay: 820.0, lines: 10 },
                LevelData { gravity: 0.1236, lock_delay: 790.0, lines: 10 },
                LevelData { gravity: 0.1775, lock_delay: 760.0, lines: 10 },
                LevelData { gravity: 0.2598, lock_delay: 730.0, lines: 10 },
                LevelData { gravity: 0.388, lock_delay: 700.0, lines: 10 },
                LevelData { gravity: 0.59, lock_delay: 670.0, lines: 10 },
                LevelData { gravity: 0.92, lock_delay: 640.0, lines: 10 },
                LevelData { gravity: 1.46, lock_delay: 610.0, lines: 10 },
                LevelData { gravity: 2.36, lock_delay: 580.0, lines: 10 },
                LevelData { gravity: 3.91, lock_delay: 550.0, lines: 10 },
                LevelData { gravity: 6.61, lock_delay: 520.0, lines: 10 },
                LevelData { gravity: 11.43, lock_delay: 490.0, lines: 10 },
                LevelData { gravity: 20.0, lock_delay: 460.0, lines: 0 },
            ]
        }
    }
}
