use core::hash::{BuildHasher, Hash};
use super::{Equivalent, IndexSet};
use crate::map::MutableKeys;
pub trait MutableValues: private::Sealed {
    type Value;
    fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut Self::Value)>
    where
        Q: ?Sized + Hash + Equivalent<Self::Value>;
    fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value>;
    fn retain2<F>(&mut self, keep: F)
    where
        F: FnMut(&mut Self::Value) -> bool;
}
trait Entries {
    type Entry;
    fn into_entries(self) -> Vec<Self::Entry>;
    fn as_entries(&self) -> &[Self::Entry];
    fn as_entries_mut(&mut self) -> &mut [Self::Entry];
    fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Self::Entry]);
}
pub trait Sealed {}
#[cfg(not(feature = "std"))]
pub struct IndexSet<T, S> {
    pub(crate) map: IndexMap<T, (), S>,
}
#[cfg(not(feature = "std"))]
pub struct IndexMap<K, V, S> {
    pub(crate) core: IndexMapCore<K, V>,
    hash_builder: S,
}
impl<T, S> MutableValues for IndexSet<T, S>
where
    S: BuildHasher,
{
    type Value = T;
    fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut T)>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {}
    fn get_index_mut2(&mut self, index: usize) -> Option<&mut T> {
        match self.map.get_index_mut2(index) {
            Some((value, ())) => Some(value),
            None => None,
        }
    }
    fn retain2<F>(&mut self, mut keep: F)
    where
        F: FnMut(&mut T) -> bool,
    {}
}
