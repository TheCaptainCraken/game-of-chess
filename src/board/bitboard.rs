use std::fmt::Display;

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bit_board = String::new();
        for i in 0..64 {
            if i % 8 == 0 {
                bit_board.push_str("\n");
            }
            if self.0 & (1 << i) != 0 {
                bit_board.push_str("1 ");
            } else {
                bit_board.push_str("0 ");
            }
        }
        write!(f, "{}", bit_board)
    }
}
