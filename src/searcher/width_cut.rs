use std::marker::Send;
use std::sync::Arc;
use std::thread;

use crate::db::DB;
use crate::game::Board;

pub struct WidthCutSearcher<D>
where
    D: DB<Vec<u8>, i8> + Send + Sync,
{
    worker_num: usize,
    max_width: usize,
    db: Arc<D>,
}

fn search_line<D>(db: &D, board: Board, max_width: usize) -> i8
where
    D: DB<Vec<u8>, i8>,
{
    let key = board.get_key();
    if let Some(score) = db.get(&key) {
        return board.get_score() + score;
    }
    if board.is_finished() {
        return board.get_score();
    }
    let mut best = -128;
    let mut next_list = board.list_next().drain().collect::<Vec<_>>();
    next_list.sort_by_key(|b| b.get_score());
    for next in next_list.into_iter().take(max_width) {
        let score = -search_line(db, next, max_width);
        if best < score {
            best = score;
        }
    }
    db.set(key, best - board.get_score());
    best
}

fn search<D>(db: Arc<D>, board: Board, max_width: usize)
where
    D: DB<Vec<u8>, i8>,
{
    search_line(db.as_ref(), board, max_width);
}

impl<D> WidthCutSearcher<D>
where
    D: DB<Vec<u8>, i8> + Send + Sync + 'static,
{
    pub fn new(db: D, worker_num: usize, max_width: usize) -> WidthCutSearcher<D> {
        WidthCutSearcher {
            worker_num,
            max_width,
            db: Arc::new(db),
        }
    }

    pub fn search(&self, board: &Board) -> i8 {
        let mut workers = Vec::new();
        for _ in 0..self.worker_num {
            let db = self.db.clone();
            let board = board.clone();
            let width = self.max_width;
            let t = thread::spawn(move || search(db, board, width));
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
