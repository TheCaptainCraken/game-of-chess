pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    #[inline]
    pub fn as_index(&self) -> usize {
        match self {
            Piece::Pawn => 0,
            Piece::Knight => 1,
            Piece::Bishop => 2,
            Piece::Rook => 3,
            Piece::Queen => 4,
            Piece::King => 5,
        }
    }

    #[inline]
    pub fn as_u8(&self) -> u8 {
        match self {
            Piece::Pawn => 0,
            Piece::Knight => 1,
            Piece::Bishop => 2,
            Piece::Rook => 3,
            Piece::Queen => 4,
            Piece::King => 5,
        }
    }

    #[inline]
    pub fn from_u8(value: u8) -> Piece {
        match value {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => panic!("Invalid piece value: {}", value),
        }
    }
}
