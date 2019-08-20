use mancala_full_search::{search_score, Board, CompactKey};
use typenum::*;

fn main() {
    let board = Board::<U5, U3>::new(true);
    let key = board.key();
    let (pits, seeds, stealing) = board.triple();
    let db = search_score(board, 4, 1024);
    println!(
        "{} {} {} {} {}",
        stealing,
        pits,
        seeds,
        db.len(),
        db.get(&key).unwrap()
    );
}
