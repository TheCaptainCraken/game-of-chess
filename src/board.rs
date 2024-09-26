pub mod bitboard;

use crate::game_constants;
use crate::piece;
use crate::side;
use bitboard::BitBoard;

pub struct Board {
    piece_bitboards: [[BitBoard; game_constants::NUM_PIECES]; game_constants::NUM_SIDES],
    color_bitboards: [BitBoard; game_constants::NUM_SIDES],
    piece_list: [u8; game_constants::NUM_SQUARES],
}

impl Board {
    #[inline]
    pub fn get_piece_bitboard(&self, side: side::Side, piece: piece::Piece) -> BitBoard {
        self.piece_bitboards[side as usize][piece as usize]
    }

    #[inline]
    pub fn get_color_bitboard(&self, side: side::Side) -> BitBoard {
        self.color_bitboards[side as usize]
    }

    #[inline]
    pub fn get_piece_at_rank_and_file(&self, rank: u8, file: u8) -> piece::Piece {
        piece::Piece::from_u8(self.piece_list[(rank * 8 + file) as usize])
    }

    #[inline]
    pub fn get_piece_at(&self, square: u8) -> piece::Piece {
        piece::Piece::from_u8(self.piece_list[square as usize])
    }
}
