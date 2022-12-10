#[derive(Clone, Copy)]
pub struct LevelData {
    pub gravity: f32,
    pub lock_delay: f32,
    pub lines: i32,
}

impl Default for LevelData {
    fn default() -> Self {
        Self {
            gravity: 0.0,
            lock_delay: 1000.0,
            lines: 0, // 0 = Infinity
        }
    }
}
