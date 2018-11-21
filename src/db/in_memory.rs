use std::collections::HashMap;
use std::hash::Hash;

use crate::db::DB;

pub struct InMemoryDB<K, V>
where
    K: Hash + Eq,
{
    db: HashMap<K, V>,
}

impl<K, V> DB<K, V> for InMemoryDB<K, V>
where
    K: Hash + Eq,
{
    fn get(&self, key: &K) -> Option<&V> {
        self.db.get(key)
    }
    fn set(&mut self, key: K, value: V) {
        self.db.insert(key, value);
    }
    fn len(&self) -> usize {
        self.db.len()
    }
}

impl<K, V> InMemoryDB<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> InMemoryDB<K, V> {
        InMemoryDB { db: HashMap::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let mut db = InMemoryDB::<String, i32>::new();
        db.set("a".to_string(), 1);
        assert_eq!(db.get(&"a".to_string()), Some(&1));
        assert_eq!(db.get(&"b".to_string()), None);
    }
}
