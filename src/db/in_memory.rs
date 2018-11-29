use std::collections::HashMap;
use std::hash::Hash;

pub struct InMemoryDB<K, V> {
    db: HashMap<K, V>,
}

impl<K, V> InMemoryDB<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    pub fn new() -> InMemoryDB<K, V> {
        InMemoryDB { db: HashMap::new() }
    }
    pub fn get(&self, key: &K) -> Option<V> {
        self.db.get(key).and_then(|v| Some(v.clone()))
    }
    pub fn set(&mut self, key: K, value: V) {
        self.db.insert(key, value);
    }
    pub fn len(&self) -> usize {
        self.db.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let mut db = InMemoryDB::new();
        db.set(vec![b'a'], 1);
        assert_eq!(db.get(&vec![b'a']), Some(1));
        assert_eq!(db.get(&vec![b'b']), None);
        assert_eq!(db.len(), 1);
    }
}
