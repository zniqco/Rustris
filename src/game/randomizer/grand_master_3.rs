use super::*;

const PIECE_ORDER: &[PieceType] = &[
    PieceType::I, PieceType::Z, PieceType::S, PieceType::J, PieceType::L, PieceType::O, PieceType::T
];

const FIRST_PIECES: &[PieceType] = &[
    PieceType::L, PieceType::J, PieceType::I, PieceType::T
];

const ROLL_COUNT: usize = 6;

pub struct GrandMaster3 {
    seed: u32,
    pool: Vec<PieceType>,
    history: Vec<PieceType>,
    histogram: [u64; 7]
}

impl GrandMaster3 {
    pub fn new(seed: u64) -> RandomizerType {
        let mut instance = Self {
            seed: (seed & 0xFFFFFFFF) as u32,
            pool: Vec::new(),
            history: Vec::new(),
            histogram: [4; 7],
        };

        for i in 0..35 {
            instance.pool.push(PIECES[(i / 5) as usize]);
        }

        let rand = instance.rand();

        instance.history.push(FIRST_PIECES[rand % FIRST_PIECES.len()]);
        instance.history.push(PieceType::Z);
        instance.history.push(PieceType::S);
        instance.history.push(PieceType::S);

        instance.into()
    }

    pub fn rand(&mut self) -> usize {
        self.seed = 0x41C64E6Du32.wrapping_mul(self.seed).wrapping_add(12345);
        
        ((self.seed >> 10) & 0x7FFF) as usize
    }
}

impl Randomizer for GrandMaster3 {
    fn pop(&mut self) -> PieceType {
        let mut position = 0usize;
        let mut piece = PieceType::I;
        let mut droughted_piece = PieceType::I;
        let mut highscore = 0u64;

        for roll in 0..ROLL_COUNT + 1 {
            position = self.rand() % self.pool.len();
            piece = self.pool[position];

            if roll == ROLL_COUNT || !self.history.contains(&piece) {
                break;
            }

            for j in 0..7 {
                if highscore < self.histogram[j] {
                    highscore = self.histogram[j];
                    droughted_piece = PIECE_ORDER[j];
                }
            }

            self.pool[position] = droughted_piece;
        }

        for j in 0..7 {
            if PIECE_ORDER[j] == droughted_piece {
                self.histogram[j] = 0;
            } else {
                self.histogram[j] += 1;
            }
        }

        for j in 0..7 {
            if highscore < self.histogram[j] {
                highscore = self.histogram[j];
                droughted_piece = PIECE_ORDER[j];
            }
        }

        self.pool[position] = droughted_piece;
        
        self.history.remove(self.history.len() - 1);
        self.history.push(piece);

        piece
    }
}
