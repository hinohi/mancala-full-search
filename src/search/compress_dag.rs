use std::collections::{HashMap, HashSet};

use generic_array::ArrayLength;
use typenum::Unsigned;

use crate::{Board, CompactKey};

type DB<P, S, V> = HashMap<<Board<P, S> as CompactKey>::Key, V>;

pub fn compress_dag<P, S, V>(root: Board<P, S>, db: &DB<P, S, V>, depth: usize) -> DB<P, S, V>
where
    P: ArrayLength<u8> + Clone,
    S: Unsigned + Clone,
    V: Clone,
    Board<P, S>: CompactKey,
{
    let mut compressed = HashMap::new();
    let mut leaf = vec![root];
    while leaf.len() > 0 {
        let mut mem = HashSet::new();
        for _ in 0..depth {
            let mut next_list = Vec::new();
            for board in leaf {
                for next in board.list_next() {
                    let key = next.key();
                    if compressed.contains_key(&key) {
                        continue;
                    }
                    if mem.insert(key) {
                        next_list.push(next);
                    }
                }
            }
            leaf = next_list;
        }
        eprintln!("compress: {} -> {}", mem.len(), leaf.len());
        for board in leaf.iter() {
            let key = board.key();
            let v = db.get(&key).unwrap().clone();
            compressed.insert(key, v);
        }
    }
    compressed
}
