use std::collections::VecDeque;
use rand::prelude::*;
use super::*;

pub struct DoubleBag {
    rng: StdRng,
    queue: VecDeque<PieceType>,
}

impl DoubleBag {
    pub fn new(seed: u64) -> RandomizerType {
        let instance = Self {
            rng: StdRng::seed_from_u64(seed),
            queue: VecDeque::new(),
        };

        instance.into()
    }
}

impl Randomizer for DoubleBag {
    fn pop(&mut self) -> PieceType {
        if self.queue.len() == 0 {
            let mut next = vec![];

            next.extend_from_slice(&PIECES);
            next.extend_from_slice(&PIECES);
            next.shuffle(&mut self.rng);

            for entry in next {
                self.queue.push_back(entry);
            }
        }

        self.queue.pop_front().unwrap()
    }
}
