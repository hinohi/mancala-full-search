use std::marker::Send;
use std::sync::Arc;
use std::thread;

use crate::db::DB;
use crate::game::Board;

pub struct MultiSearcher<D>
where
    D: DB<Vec<u8>, i8> + Send + Sync,
{
    n: usize,
    db: Arc<D>,
}

fn search_line<D>(db: &D, board: Board) -> i8
where
    D: DB<Vec<u8>, i8>,
{
    let key = board.get_key();
    if let Some(score) = db.get(&key) {
        return board.get_score() + score;
    }
    if board.is_finished() {
        db.set(key, 0);
        return board.get_score();
    }
    let mut best = -128;
    for next in board.list_next() {
        let score = -search_line(db, next);
        if best < score {
            best = score;
        }
    }
    db.set(key, best - board.get_score());
    best
}

fn search<D>(db: Arc<D>, board: Board)
where
    D: DB<Vec<u8>, i8>,
{
    search_line(db.as_ref(), board);
}

impl<D> MultiSearcher<D>
where
    D: DB<Vec<u8>, i8> + Send + Sync + 'static,
{
    pub fn new(n: usize, db: D) -> MultiSearcher<D> {
        MultiSearcher {
            n,
            db: Arc::new(db),
        }
    }

    pub fn search(&self, board: &Board) -> i8 {
        let mut workers = Vec::new();
        for _ in 0..self.n {
            let db = self.db.clone();
            let board = board.clone();
            let t = thread::spawn(move || search(db, board));
            workers.push(t);
        }
        for t in workers {
            t.join().unwrap();
        }
        let key = board.get_key();
        self.db.get(&key).unwrap()
    }

    pub fn len(&self) -> usize {
        self.db.len()
    }

    pub fn get_db(&self) -> Arc<D> {
        self.db.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::InMemoryDB;

    #[test]
    fn search_11() {
        let b = Board::new(1, 1);
        let db = InMemoryDB::new(1);
        let s = MultiSearcher::new(1, db);
        assert_eq!(s.search(&b), 1);
        assert_eq!(s.db.len(), 2);
    }

    #[test]
    fn search_12() {
        let b = Board::new(1, 2);
        let db = InMemoryDB::new(1);
        let s = MultiSearcher::new(1, db);
        assert_eq!(s.search(&b), 1);
        assert_eq!(s.db.len(), 2);
    }

    #[test]
    fn search_15() {
        let b = Board::new(1, 5);
        let db = InMemoryDB::new(1);
        let s = MultiSearcher::new(1, db);
        assert_eq!(s.search(&b), -2);
        assert_eq!(s.db.len(), 3);
    }

    #[test]
    fn search_21() {
        let b = Board::new(2, 1);
        let db = InMemoryDB::new(1);
        let s = MultiSearcher::new(1, db);
        assert_eq!(s.search(&b), 1);
        assert_eq!(s.db.len(), 8);
    }

    #[test]
    fn search_32() {
        let b = Board::new(3, 2);
        let db = InMemoryDB::new(1);
        let s = MultiSearcher::new(1, db);
        assert_eq!(s.search(&b), 4);
        assert_eq!(s.db.len(), 1239);
    }

    #[test]
    fn multi_search_32() {
        let b = Board::new(3, 2);
        let db = InMemoryDB::new(1);
        let s = MultiSearcher::new(4, db);
        assert_eq!(s.search(&b), 4);
        assert_eq!(s.db.len(), 1239);
    }
}
