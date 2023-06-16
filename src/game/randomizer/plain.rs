use rand::prelude::*;
use super::*;

pub struct Plain {
    rng: StdRng,
}

impl Plain {
    pub fn new(seed: u64) -> Randomizer {
        let instance = Self {
            rng: StdRng::seed_from_u64(seed),
        };

        instance.into()
    }
}

impl RandomizerImpl for Plain {
    fn pop(&mut self) -> PieceType {
        *PIECES.choose(&mut self.rng).unwrap()
    }
}
