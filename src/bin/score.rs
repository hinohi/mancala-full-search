use std::collections::HashMap;

use mancala_full_search::{compress_dag, search_score, Board, CompactKey};
use typenum::*;

fn main() {
    let board = Board::<U5, U2>::new(false);
    let key = board.key();
    let (pits, seeds, stealing) = board.triple();
    let db: HashMap<_, _> = search_score(board.clone(), 4, 1024).into();
    println!("{} pits={} seeds={}", stealing, pits, seeds);
    println!("len={} score={}", db.len(), db.get(&key).unwrap());

    let depth = 8;
    let compressed = compress_dag(board, &db, depth);
    println!("depth={} compressed={}", depth, compressed.len());
}
