use std::cmp::Ordering;
use std::collections::hash_map::RandomState;
use std::fmt::{self, Display, Formatter};
use std::hash::BuildHasherDefault;
use std::ops::{Add, Neg, Sub};

use fnv::{FnvBuildHasher, FnvHasher};
use generic_array::ArrayLength;
use locked_hash::LockedHashMap;
use typenum::Unsigned;

use crate::{Board, CompactKey};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Settlement {
    Win(u8),
    Lose(u8),
    Draw,
}

impl Ord for Settlement {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // 勝つなら早いほうがいい
            (Settlement::Win(a), Settlement::Win(b)) => b.cmp(a),
            // 負けるなら遅いほうがいい
            (Settlement::Lose(a), Settlement::Lose(b)) => a.cmp(b),
            (Settlement::Win(_), _) => Ordering::Greater,
            (Settlement::Lose(_), _) => Ordering::Less,
            (Settlement::Draw, Settlement::Win(_)) => Ordering::Less,
            (Settlement::Draw, Settlement::Lose(_)) => Ordering::Greater,
            (Settlement::Draw, Settlement::Draw) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Settlement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Neg for Settlement {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Settlement::Win(t) => Settlement::Lose(t),
            Settlement::Lose(t) => Settlement::Win(t),
            Settlement::Draw => Settlement::Draw,
        }
    }
}

impl Add<u8> for Settlement {
    type Output = Self;

    fn add(self, other: u8) -> Self::Output {
        match self {
            Settlement::Win(n) => Settlement::Win(n + other),
            Settlement::Lose(n) => Settlement::Lose(n + other),
            Settlement::Draw => Settlement::Draw,
        }
    }
}

impl Sub<u8> for Settlement {
    type Output = Self;

    fn sub(self, other: u8) -> Self::Output {
        match self {
            Settlement::Win(n) => Settlement::Win(n - other),
            Settlement::Lose(n) => Settlement::Lose(n - other),
            Settlement::Draw => Settlement::Draw,
        }
    }
}

impl Display for Settlement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Settlement::Win(t) => write!(f, "Win({})", t),
            Settlement::Lose(t) => write!(f, "Lose({})", t),
            Settlement::Draw => write!(f, "Draw"),
        }
    }
}

impl Settlement {
    const fn min() -> Settlement {
        Settlement::Lose(0)
    }
}

type DB<P, S> = LockedHashMap<
    <Board<P, S> as CompactKey>::Key,
    Settlement,
    RandomState,
    BuildHasherDefault<FnvHasher>,
>;

fn search_worker<P, S>(db: &DB<P, S>, board: Board<P, S>, depth: u8) -> Settlement
where
    P: ArrayLength<u8> + Clone,
    S: Unsigned + Clone,
    Board<P, S>: CompactKey,
    <Board<P, S> as CompactKey>::Key: Send + Sync + Display,
{
    let key = board.key();
    if let Some(score) = db.get(&key) {
        return score + depth;
    }
    if board.is_finished() {
        db.insert(key, Settlement::Lose(0));
        return Settlement::Lose(depth);
    }
    let mut best = Settlement::min();
    for next in board.list_next() {
        let score = -search_worker(db, next, depth + 1);
        if best < score {
            best = score;
        }
    }
    db.insert(key, best - depth);
    best
}

pub fn search_clean<P, S>(board: Board<P, S>, threads: usize, div: usize) -> DB<P, S>
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
                search_worker(&db, board, 0);
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
    fn settlement() {
        assert!(Settlement::Win(3) < Settlement::Win(2));
        assert!(Settlement::Win(1) > Settlement::Win(2));
        assert!(Settlement::Draw < Settlement::Win(2));
        assert!(Settlement::Lose(1) < Settlement::Win(2));
        assert!(Settlement::Lose(3) < Settlement::Win(2));

        assert!(Settlement::Win(1) > Settlement::Lose(2));
        assert!(Settlement::Win(3) > Settlement::Lose(2));
        assert!(Settlement::Draw > Settlement::Lose(2));
        assert!(Settlement::Lose(1) < Settlement::Lose(2));
        assert!(Settlement::Lose(3) > Settlement::Lose(2));

        assert_eq!(Settlement::Win(4), Settlement::Win(4));
        assert_eq!(Settlement::Lose(4), Settlement::Lose(4));
        assert_eq!(Settlement::Draw, Settlement::Draw);
    }

    #[test]
    fn search_1_1() {
        let board = Board::<U1, U1>::new(true);
        let key = board.key();
        let db = search_clean(board, 2, 4);
        assert_eq!(db.len(), 2);
        assert_eq!(db.get(&key), Some(Settlement::Win(1)));
    }

    #[test]
    fn search_1_2() {
        let board = Board::<U1, U1>::new(true);
        let key = board.key();
        let db = search_clean(board, 2, 4);
        assert_eq!(db.len(), 2);
        assert_eq!(db.get(&key), Some(Settlement::Win(1)));
    }

    #[test]
    fn score_2_1_true() {
        let board = Board::<U2, U1>::new(true);
        let key = board.key();
        let db = search_clean(board, 4, 16);
        assert_eq!(db.len(), 7);
        assert_eq!(db.get(&key), Some(Settlement::Win(1)));
    }

    #[test]
    fn score_2_1_false() {
        let board = Board::<U2, U1>::new(false);
        let key = board.key();
        let db = search_clean(board, 4, 16);
        assert_eq!(db.len(), 11);
        assert_eq!(db.get(&key), Some(Settlement::Win(3)));
    }
}
