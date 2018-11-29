use std::boxed::Box;

use crate::db::NoLockDB;
use crate::game::Board;

type SDB = NoLockDB<Vec<u8>, i8>;

pub struct Searcher {
    db: SDB,
}

impl Searcher {
    pub fn new(db: SDB) -> Searcher {
        Searcher { db }
    }

    pub fn search(&mut self, board: &Board) -> i8 {
        let key = board.get_key();
        if let Some(score) = self.db.get(&key) {
            return board.get_score() + score;
        }
        if board.is_finished() {
            self.db.set(key, 0);
            return board.get_score();
        }
        let mut best = -128;
        for next in board.list_next() {
            let score = -self.search(&next);
            if best < score {
                best = score;
            }
        }
        self.db.set(key, best - board.get_score());
        best
    }

    pub fn len(&self) -> usize {
        self.db.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::NoLockDB;

    #[test]
    fn search_11() {
        let b = Board::new(1, 1);
        let db = NoLockDB::new();
        let mut s = Searcher::new(db);
        assert_eq!(s.search(&b), 1);
        assert_eq!(s.db.len(), 2);
    }

    #[test]
    fn search_12() {
        let b = Board::new(1, 2);
        let db = NoLockDB::new();
        let mut s = Searcher::new(db);
        assert_eq!(s.search(&b), 1);
        assert_eq!(s.db.len(), 2);
    }

    #[test]
    fn search_15() {
        let b = Board::new(1, 5);
        let db = NoLockDB::new();
        let mut s = Searcher::new(db);
        assert_eq!(s.search(&b), -2);
        assert_eq!(s.db.len(), 3);
    }

    #[test]
    fn search_21() {
        let b = Board::new(2, 1);
        let db = NoLockDB::new();
        let mut s = Searcher::new(db);
        assert_eq!(s.search(&b), 1);
        assert_eq!(s.db.len(), 8);
    }

    #[test]
    fn search_32() {
        let b = Board::new(3, 2);
        let db = NoLockDB::new();
        let mut s = Searcher::new(db);
        assert_eq!(s.search(&b), 4);
        assert_eq!(s.db.len(), 1239);
    }
}
