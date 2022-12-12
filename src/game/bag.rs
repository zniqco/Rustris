use super::*;
use rand::prelude::*;

const MINIMUM_COUNT: usize = 7;

pub struct Bag {
    pieces: Vec<PieceType>,
    rng: StdRng,
}

impl Bag {
    pub fn new(seed: Option<u64>) -> Self {
        let mut instance = Self {
            pieces: Vec::new(),
            rng: StdRng::seed_from_u64(match seed {
                Some(x) => x,
                None => Self::get_timestamp(),
            }),
        };

        instance.fill_to_minimum();

        instance
    }

    pub fn pop(&mut self) -> PieceType {
        self.fill_to_minimum();

        let piece = self.pieces[0];

        self.pieces.remove(0);

        piece
    }

    pub fn push_front(&mut self, piece_type: PieceType) {
        self.pieces.insert(0, piece_type);
    }

    pub fn get(&self, index: i32) -> PieceType {
        self.pieces[index as usize]
    }

    fn fill_to_minimum(&mut self) {
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
    }

    fn get_timestamp() -> u64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64
    }
}