use std::collections::HashMap;

use crate::db::DB;

pub struct InMemoryDB<V> {
    db: HashMap<Vec<u8>, V>,
}

impl<V> DB<V> for InMemoryDB<V> {
    fn get(&self, key: &Vec<u8>) -> Option<&V> {
        self.db.get(key)
    }
    fn set(&mut self, key: Vec<u8>, value: V) {
        self.db.insert(key, value);
    }
    fn len(&self) -> usize {
        self.db.len()
    }
}

impl<V> InMemoryDB<V> {
    pub fn new() -> InMemoryDB<V> {
        InMemoryDB { db: HashMap::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let mut db = InMemoryDB::new();
        db.set(vec![b'a'], 1);
        assert_eq!(db.get(&vec![b'a']), Some(&1));
        assert_eq!(db.get(&vec![b'b']), None);
    }
}
