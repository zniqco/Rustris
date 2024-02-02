use super::*;

pub const MODE_ZEN: Mode = Mode {
    das: None,
    arr: None,
    sdf: None,
    width: 10,
    height: 20,
    randomizer: RandomizerType::SingleBag,
    rotation: None,
    levels: &[
        LevelData { gravity: 0.0, lock_delay: 100.0, lines: 0 },
    ],
};
