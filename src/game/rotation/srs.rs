use crate::game::PieceType;
use super::*;

pub const SRS_SHAPE_Z: Shape = [
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
];

pub const SRS_SHAPE_L: Shape = [
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
];

pub const SRS_SHAPE_O: Shape = [
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
];

pub const SRS_SHAPE_S: Shape = [
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
];

pub const SRS_SHAPE_I: Shape = [
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
];

pub const SRS_SHAPE_J: Shape = [
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
];

pub const SRS_SHAPE_T: Shape = [
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
];

pub const SRS_KICK_ZLSJT: Kick = Kick {
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

pub const SRS_KICK_I: Kick = Kick {
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
    fn piece(&self, piece: PieceType) -> PieceData {
        match piece {
            PieceType::Z => PieceData {
                shape: &SRS_SHAPE_Z,
                kick: &SRS_KICK_ZLSJT,
                block: BlockType::Red,
                spawn_offset: (-2, -2),
                preview_offset: (0.0, 0.5),
            },
            PieceType::L => PieceData {
                shape: &SRS_SHAPE_L,
                kick: &SRS_KICK_ZLSJT,
                block: BlockType::Orange,
                spawn_offset: (-2, -2),
                preview_offset: (0.0, 0.5),
            },
            PieceType::O => PieceData {
                shape: &SRS_SHAPE_O,
                kick: &KICK_NONE,
                block: BlockType::Yellow,
                spawn_offset: (-1, -1),
                preview_offset: (0.0, 0.0),
            },
            PieceType::S => PieceData {
                shape: &SRS_SHAPE_S,
                kick: &SRS_KICK_ZLSJT,
                block: BlockType::Green,
                spawn_offset: (-2, -2),
                preview_offset: (0.0, 0.5),
            },
            PieceType::I => PieceData {
                shape: &SRS_SHAPE_I,
                kick: &SRS_KICK_I,
                block: BlockType::Cyan,
                spawn_offset: (-2, -3),
                preview_offset: (0.0, 0.5),
            },
            PieceType::J => PieceData {
                shape: &SRS_SHAPE_J,
                kick: &SRS_KICK_ZLSJT,
                block: BlockType::Blue,
                spawn_offset: (-2, -2),
                preview_offset: (0.0, 0.5),
            },
            PieceType::T => PieceData {
                shape: &SRS_SHAPE_T,
                kick: &SRS_KICK_ZLSJT,
                block: BlockType::Purple,
                spawn_offset: (-2, -2),
                preview_offset: (0.0, 0.5),
            },
        }
    }
}
