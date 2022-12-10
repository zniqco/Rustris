use super::*;
use rand::prelude::*;

const MINIMUM_COUNT: usize = 7;

pub struct Bag {
    pieces: Vec<PieceType>,
    rng: StdRng,
}

impl Bag {
    pub fn new(seed: Option<u64>) -> Self {
        Self {
            pieces: Vec::new(),
            rng: StdRng::seed_from_u64(match seed {
                Some(x) => x,
                None => Self::get_timestamp(),
            }),
        }
    }

    pub fn pop(&mut self) -> PieceType {
        let piece = self.get(0);

        self.pieces.remove(0);

        piece
    }

    pub fn push_front(&mut self, piece_type: PieceType) {
        self.pieces.insert(0, piece_type);
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

    fn get_timestamp() -> u64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64
    }
}
