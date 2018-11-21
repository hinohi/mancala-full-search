use std::hash::Hash;

pub trait DB<K, V>
where
    K: Hash + Eq,
{
    fn get(&self, key: &K) -> Option<&V>;
    fn set(&mut self, key: K, value: V);
    fn len(&self) -> usize;
}
