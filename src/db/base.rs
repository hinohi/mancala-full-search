pub trait DB<K, V> {
    fn get(&self, key: &K) -> Option<V>;
    fn set(&self, key: K, value: V);
    fn len(&self) -> usize;
}
