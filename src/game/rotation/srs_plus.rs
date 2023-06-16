use crate::game::PieceType;
use super::*;

/* SRS+ From TETR.IO */

pub const SRS_PLUS_KICK_ZLSJT: KickData = KickData {
    cw: [
        &[(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
        &[(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
        &[(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
        &[(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    ],
    ccw: [
        &[(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
        &[(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
        &[(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
        &[(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    ],
    flip: [
        &[(0, 0), (0, 1), (1, 1), (-1, 1), (1, 0), (-1, 0)],
        &[(0, 0), (1, 0), (1, 2), (1, 1), (0, 2), (0, 1)],
        &[(0, 0), (0, -1), (-1, -1), (1, -1), (-1, 0), (1, 0)],
        &[(0, 0), (-1, 0), (-1, 2), (-1, 1), (0, 2), (0, 1)],
    ]
};

pub const SRS_PLUS_KICK_I: KickData = KickData {
    cw: [
        &[(0, 0), (1, 0), (-2, 0), (-2, -1), (1, 2)],
        &[(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
        &[(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
        &[(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    ],
    ccw: [
        &[(0, 0), (-1, 0), (2, 0), (-1, -2), (2, 1)],
        &[(0, 0), (-2, 0), (1, 0), (-2, 1), (1, -2)],
        &[(0, 0), (1, 0), (-2, 0), (1, 2), (-2, -1)],
        &[(0, 0), (-1, 0), (2, 0), (2, -1), (-1, 2)],
    ],
    flip: [
        &[(0, 0), (0, 1)],
        &[(0, 0), (1, 0)],
        &[(0, 0), (0, -1)],
        &[(0, 0), (-1, 0)],
    ],
};

pub struct SRSPlus;

impl SRSPlus {
    pub fn new() -> Rotation {
        Self.into()
    }
}

impl RotationImpl for SRSPlus {
    fn blocks(&self, piece: PieceType) -> &'static PieceData {
        match piece {
            PieceType::Z => &SRS_PIECE_Z,
            PieceType::L => &SRS_PIECE_L,
            PieceType::O => &SRS_PIECE_O,
            PieceType::S => &SRS_PIECE_S,
            PieceType::I => &SRS_PIECE_I,
            PieceType::J => &SRS_PIECE_J,
            PieceType::T => &SRS_PIECE_T,
        }
    }

    fn kicks(&self, piece: PieceType) -> &'static KickData {
        match piece {
            PieceType::Z | PieceType::L | PieceType::S | PieceType::J | PieceType::T => &SRS_PLUS_KICK_ZLSJT,
            PieceType::I => &SRS_PLUS_KICK_I,
            PieceType::O => &KICK_NONE,
        }
    }
}
