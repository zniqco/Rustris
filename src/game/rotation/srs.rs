use crate::game::PieceType;
use super::*;

pub const SRS_PIECE_Z: PieceData = PieceData {
    shape: [
        &[
            &[1, 1, 0],
            &[0, 1, 1],
            &[0, 0, 0],
        ],
        &[
            &[0, 0, 1],
            &[0, 1, 1],
            &[0, 1, 0],
        ],
        &[
            &[0, 0, 0],
            &[1, 1, 0],
            &[0, 1, 1],
        ],
        &[
            &[0, 1, 0],
            &[1, 1, 0],
            &[1, 0, 0],
        ]
    ],
    block: BlockType::Red,
    spawn_offset: (-2, -2),
    preview_offset: (0.0, 0.5)
};

pub const SRS_PIECE_L: PieceData = PieceData {
    shape: [
        &[
            &[0, 0, 1],
            &[1, 1, 1],
            &[0, 0, 0],
        ],
        &[
            &[0, 1, 0],
            &[0, 1, 0],
            &[0, 1, 1],
        ],
        &[
            &[0, 0, 0],
            &[1, 1, 1],
            &[1, 0, 0],
        ],
        &[
            &[1, 1, 0],
            &[0, 1, 0],
            &[0, 1, 0],
        ]
    ],
    block: BlockType::Orange,
    spawn_offset: (-2, -2),
    preview_offset: (0.0, 0.5)
};

pub const SRS_PIECE_O: PieceData = PieceData {
    shape: [
        &[
            &[1, 1],
            &[1, 1],
        ],
        &[
            &[1, 1],
            &[1, 1],
        ],
        &[
            &[1, 1],
            &[1, 1],
        ],
        &[
            &[1, 1],
            &[1, 1],
        ]
    ],
    block: BlockType::Yellow,
    spawn_offset: (-1, -1),
    preview_offset: (0.0, 0.0)
};

pub const SRS_PIECE_S: PieceData = PieceData {
    shape: [
        &[
            &[0, 1, 1],
            &[1, 1, 0],
            &[0, 0, 0],
        ],
        &[
            &[0, 1, 0],
            &[0, 1, 1],
            &[0, 0, 1],
        ],
        &[
            &[0, 0, 0],
            &[0, 1, 1],
            &[1, 1, 0],
        ],
        &[
            &[1, 0, 0],
            &[1, 1, 0],
            &[0, 1, 0],
        ]
    ],
    block: BlockType::Green,
    spawn_offset: (-2, -2),
    preview_offset: (0.0, 0.5)
};

pub const SRS_PIECE_I: PieceData = PieceData {
    shape: [
        &[
            &[0, 0, 0, 0],
            &[1, 1, 1, 1],
            &[0, 0, 0, 0],
            &[0, 0, 0, 0],
        ],
        &[
            &[0, 0, 1, 0],
            &[0, 0, 1, 0],
            &[0, 0, 1, 0],
            &[0, 0, 1, 0],
        ],
        &[
            &[0, 0, 0, 0],
            &[0, 0, 0, 0],
            &[1, 1, 1, 1],
            &[0, 0, 0, 0],
        ],
        &[
            &[0, 1, 0, 0],
            &[0, 1, 0, 0],
            &[0, 1, 0, 0],
            &[0, 1, 0, 0],
        ]
    ],
    block: BlockType::Cyan,
    spawn_offset: (-2, -3),
    preview_offset: (0.0, 0.5)
};

pub const SRS_PIECE_J: PieceData = PieceData {
    shape: [
        &[
            &[1, 0, 0],
            &[1, 1, 1],
            &[0, 0, 0],
        ],
        &[
            &[0, 1, 1],
            &[0, 1, 0],
            &[0, 1, 0],
        ],
        &[
            &[0, 0, 0],
            &[1, 1, 1],
            &[0, 0, 1],
        ],
        &[
            &[0, 1, 0],
            &[0, 1, 0],
            &[1, 1, 0],
        ]
    ],
    block: BlockType::Blue,
    spawn_offset: (-2, -2),
    preview_offset: (0.0, 0.5)
};

pub const SRS_PIECE_T: PieceData = PieceData {
    shape: [
        &[
            &[0, 1, 0],
            &[1, 1, 1],
            &[0, 0, 0],
        ],
        &[
            &[0, 1, 0],
            &[0, 1, 1],
            &[0, 1, 0],
        ],
        &[
            &[0, 0, 0],
            &[1, 1, 1],
            &[0, 1, 0],
        ],
        &[
            &[0, 1, 0],
            &[1, 1, 0],
            &[0, 1, 0],
        ]
    ],
    block: BlockType::Purple,
    spawn_offset: (-2, -2),
    preview_offset: (0.0, 0.5)
};

pub const SRS_KICK_ZLSJT: KickData = KickData {
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
        &[(0, 0)],
        &[(0, 0)],
        &[(0, 0)],
        &[(0, 0)],
    ]
};

pub const SRS_KICK_I: KickData = KickData {
    cw: [
        &[(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
        &[(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
        &[(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
        &[(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    ],
    ccw: [
        &[(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
        &[(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
        &[(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
        &[(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    ],
    flip: [
        &[(0, 0)],
        &[(0, 0)],
        &[(0, 0)],
        &[(0, 0)],
    ],
};

pub struct SRS;

impl SRS {
    pub fn new() -> Rotation {
        Self.into()
    }
}

impl RotationImpl for SRS {
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
            PieceType::Z | PieceType::L | PieceType::S | PieceType::J | PieceType::T => &SRS_KICK_ZLSJT,
            PieceType::I => &SRS_KICK_I,
            PieceType::O => &KICK_NONE,
        }
    }
}
