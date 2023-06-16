use super::*;

#[derive(Clone, Copy)]
pub struct Mode {
    pub das: Option<f32>,
    pub arr: Option<f32>,
    pub sdf: Option<f32>,
    pub width: usize,
    pub height: usize,
    pub seed: Option<u64>,
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

pub const MODE_MARATHON: Mode = Mode {
    das: None,
    arr: None,
    sdf: None,
    width: 10,
    height: 20,
    seed: None,
    randomizer: RandomizerType::SingleBag,
    rotation: None,
    levels: &[
        LevelData { gravity: 0.01667, lock_delay: 1.0, lines: 10 },
        LevelData { gravity: 0.021017, lock_delay: 0.97, lines: 10 },
        LevelData { gravity: 0.026977, lock_delay: 0.94, lines: 10 },
        LevelData { gravity: 0.035256, lock_delay: 0.91, lines: 10 },
        LevelData { gravity: 0.04693, lock_delay: 0.88, lines: 10 },
        LevelData { gravity: 0.06361, lock_delay: 0.85, lines: 10 },
        LevelData { gravity: 0.0879, lock_delay: 0.82, lines: 10 },
        LevelData { gravity: 0.1236, lock_delay: 0.79, lines: 10 },
        LevelData { gravity: 0.1775, lock_delay: 0.76, lines: 10 },
        LevelData { gravity: 0.2598, lock_delay: 0.73, lines: 10 },
        LevelData { gravity: 0.388, lock_delay: 0.70, lines: 10 },
        LevelData { gravity: 0.59, lock_delay: 0.67, lines: 10 },
        LevelData { gravity: 0.92, lock_delay: 0.64, lines: 10 },
        LevelData { gravity: 1.46, lock_delay: 0.61, lines: 10 },
        LevelData { gravity: 2.36, lock_delay: 0.58, lines: 10 },
        LevelData { gravity: 3.91, lock_delay: 0.55, lines: 10 },
        LevelData { gravity: 6.61, lock_delay: 0.52, lines: 10 },
        LevelData { gravity: 11.43, lock_delay: 0.49, lines: 10 },
        LevelData { gravity: 20.0, lock_delay: 0.46, lines: 10 },
        LevelData { gravity: 20.0, lock_delay: 0.43, lines: 10 },
    ],
};

pub const MODE_ZEN: Mode = Mode {
    das: None,
    arr: None,
    sdf: None,
    width: 10,
    height: 20,
    seed: None,
    randomizer: RandomizerType::SingleBag,
    rotation: None,
    levels: &[
        LevelData { gravity: 0.0, lock_delay: 100.0, lines: 0 },
    ],
};
