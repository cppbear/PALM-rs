use alloc::vec::{self, Vec};
pub use crate::map::IndexMap;
pub use crate::set::IndexSet;
pub use equivalent::Equivalent;
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct HashValue(usize);
impl<K, V> Bucket<K, V> {
    fn key_ref(&self) -> &K {}
    fn value_ref(&self) -> &V {}
    fn value_mut(&mut self) -> &mut V {}
    fn key(self) -> K {
        self.key
    }
    fn value(self) -> V {}
    fn key_value(self) -> (K, V) {}
    fn refs(&self) -> (&K, &V) {}
    fn ref_mut(&mut self) -> (&K, &mut V) {}
    fn muts(&mut self) -> (&mut K, &mut V) {}
}
