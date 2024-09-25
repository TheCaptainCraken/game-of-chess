use game_of_chess::board::bitboard::BitBoard;

fn main() {
    println!("{}", BitBoard::new(0xffff_0000_0000_ffff));
}
