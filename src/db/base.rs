pub trait DB<V> {
    fn get(&self, key: &Vec<u8>) -> Option<&V>;
    fn set(&mut self, key: Vec<u8>, value: V);
    fn len(&self) -> usize;
}
