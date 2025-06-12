use crate::raw::{
    Allocator, Bucket, Global, RawDrain, RawExtractIf, RawIntoIter, RawIter, RawTable,
};
use crate::{DefaultHashBuilder, Equivalent, TryReserveError};
use core::borrow::Borrow;
use core::fmt::{self, Debug};
use core::hash::{BuildHasher, Hash};
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::mem;
use core::ops::Index;
#[cfg(feature = "raw-entry")]
pub use crate::raw_entry::*;
#[allow(clippy::missing_safety_doc)]
pub unsafe trait Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()>;
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
}
pub struct OccupiedEntry<'a, K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    hash: u64,
    elem: Bucket<(K, V)>,
    table: &'a mut HashMap<K, V, S, A>,
}
pub struct Bucket<T> {
    ptr: NonNull<T>,
}
pub struct HashMap<K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) hash_builder: S,
    pub(crate) table: RawTable<(K, V), A>,
}
impl<'a, K, V, S, A: Allocator> OccupiedEntry<'a, K, V, S, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn key(&self) -> &K {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove_entry(self) -> (K, V) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get(&self) -> &V {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get_mut(&mut self) -> &mut V {
        unsafe { &mut self.elem.as_mut().1 }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn into_mut(self) -> &'a mut V {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(&mut self, value: V) -> V {
        mem::replace(self.get_mut(), value)
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove(self) -> V {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn replace_entry_with<F>(self, f: F) -> Entry<'a, K, V, S, A>
    where
        F: FnOnce(&K, V) -> Option<V>,
    {}
}
