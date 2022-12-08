use super::*;
use rand::prelude::*;

const MINIMUM_COUNT: usize = 7;

pub struct Bag {
    pieces: Vec<PieceType>,
    rng: ThreadRng,
}

impl Bag {
    pub fn new() -> Self {
        Self {
            pieces: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn pop(&mut self) -> PieceType {
        let piece = self.get(0);

        self.pieces.remove(0);

        piece
    }

    pub fn get(&mut self, index: u32) -> PieceType {
        while self.pieces.len() < MINIMUM_COUNT {
            let mut next: Vec<PieceType> = vec![
                PieceType::Z,
                PieceType::S,
                PieceType::L,
                PieceType::J,
                PieceType::I,
                PieceType::O,
                PieceType::T,
            ];

            next.shuffle(&mut self.rng);

            self.pieces.append(&mut next);
        }

        self.pieces[index as usize]
    }
}
