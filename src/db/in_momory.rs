use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::sync::RwLock;

use crate::db::DB;

pub struct InMemoryDB<K, V> {
    div: usize,
    db: Vec<RwLock<HashMap<K, V>>>,
}

impl<K, V> DB<K, V> for InMemoryDB<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn get(&self, key: &K) -> Option<V> {
        let db = self.db[self.key_slot(key)].read().unwrap();
        db.get(key).and_then(|v| Some(v.clone()))
    }
    fn set(&self, key: K, value: V) {
        let mut db = self.db[self.key_slot(&key)].write().unwrap();
        db.insert(key, value);
    }
    fn len(&self) -> usize {
        self.db.iter().map(|db| db.read().unwrap().len()).sum()
    }
}

impl<K, V> InMemoryDB<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    pub fn new(div: usize) -> InMemoryDB<K, V> {
        let mut db = Vec::with_capacity(div);
        for _ in 0..div {
            db.push(RwLock::new(HashMap::new()));
        }
        InMemoryDB { div, db }
    }

    fn key_slot(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish().wrapping_mul(3) as usize % self.div
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let db = InMemoryDB::new(1);
        db.set(vec![b'a'], 1);
        assert_eq!(db.get(&vec![b'a']), Some(1));
        assert_eq!(db.get(&vec![b'b']), None);
        assert_eq!(db.len(), 1);
    }

    #[test]
    fn multi() {
        use std::sync::Arc;
        use std::thread;

        let db = Arc::new(InMemoryDB::new(64));
        let mut w = Vec::new();
        for i in 0..10 {
            let db = db.clone();
            let t = thread::spawn(move || {
                for j in 0..10 {
                    db.set(i * 10 + j, j);
                }
            });
            w.push(t);
        }
        for t in w {
            t.join().unwrap();
        }
        for i in 0..100 {
            assert_eq!(db.get(&i), Some(i % 10));
        }
        assert_eq!(db.len(), 100);
    }
}
