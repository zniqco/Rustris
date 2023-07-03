use std::collections::VecDeque;
use rand::prelude::*;
use super::*;

pub struct DoubleBag {
    rng: StdRng,
    queue: VecDeque<PieceType>,
}

impl DoubleBag {
    pub fn new(seed: u64) -> Randomizer {
        let instance = Self {
            rng: StdRng::seed_from_u64(seed),
            queue: VecDeque::new(),
        };

        instance.into()
    }
}

impl RandomizerImpl for DoubleBag {
    fn pop(&mut self) -> PieceType {
        if self.queue.is_empty() {
            let mut next = vec![];

            next.extend_from_slice(PIECES);
            next.extend_from_slice(PIECES);
            next.shuffle(&mut self.rng);

            for entry in next {
                self.queue.push_back(entry);
            }
        }

        self.queue.pop_front().unwrap()
    }
}
