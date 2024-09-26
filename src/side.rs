pub enum Side {
    White,
    Black,
}

impl Side {
    #[inline]
    pub fn opposite(&self) -> Side {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }

    #[inline]
    pub fn as_index(&self) -> usize {
        match self {
            Side::White => 0,
            Side::Black => 1,
        }
    }
}
