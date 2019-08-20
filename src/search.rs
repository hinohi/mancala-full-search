use std::collections::hash_map::RandomState;
use std::hash::BuildHasherDefault;

use fnv::{FnvBuildHasher, FnvHasher};
use locked_hash::LockedHashMap;

use crate::{Board, CompactKey};
use generic_array::ArrayLength;
use std::fmt::Display;
use typenum::Unsigned;

type DB<P, S> =
    LockedHashMap<<Board<P, S> as CompactKey>::Key, i8, RandomState, BuildHasherDefault<FnvHasher>>;

fn search_score_worker<P, S>(db: &DB<P, S>, board: Board<P, S>) -> i8
where
    P: ArrayLength<u8> + Clone,
    S: Unsigned + Clone,
    Board<P, S>: CompactKey,
    <Board<P, S> as CompactKey>::Key: Send + Sync + Display,
{
    let key = board.key();
    if let Some(score) = db.get(&key) {
        return board.store_score() + score;
    }
    if board.is_finished() {
        db.insert(key, board.pit_score());
        return board.score();
    }
    let mut best = -128;
    for next in board.list_next() {
        let score = -search_score_worker(db, next);
        if best < score {
            best = score;
        }
    }
    db.insert(key, best - board.store_score());
    best
}

pub fn search_score<P, S>(board: Board<P, S>, threads: usize, div: usize) -> DB<P, S>
where
    P: ArrayLength<u8> + Clone + Send,
    S: Unsigned + Clone + Send,
    Board<P, S>: CompactKey,
    <Board<P, S> as CompactKey>::Key: Send + Sync + Display,
{
    let db = LockedHashMap::with_div_and_capacity_and_hasher(
        div,
        0,
        RandomState::new(),
        FnvBuildHasher::default(),
    );
    crossbeam::scope(|scope| {
        for _ in 0..threads {
            let board = board.clone();
            scope.spawn(|_| {
                search_score_worker(&db, board);
            });
        }
    })
    .unwrap();
    db
}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::{U1, U2};

    #[test]
    fn score_1_1() {
        let board = Board::<U1, U1>::new(true);
        let key = board.key();
        let db = search_score(board, 2, 4);
        assert_eq!(db.len(), 2);
        assert_eq!(db.get(&key), Some(0));
    }

    #[test]
    fn score_1_2() {
        let board = Board::<U1, U2>::new(true);
        let key = board.key();
        let db = search_score(board, 4, 16);
        assert_eq!(db.len(), 2);
        assert_eq!(db.get(&key), Some(-2));
    }

    #[test]
    fn score_2_1() {
        let board = Board::<U2, U1>::new(true);
        let key = board.key();
        let db = search_score(board, 4, 16);
        assert_eq!(db.len(), 7);
        assert_eq!(db.get(&key), Some(2));
    }
}
