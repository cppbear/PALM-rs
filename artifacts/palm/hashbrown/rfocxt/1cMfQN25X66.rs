use crate::hash_map::{equivalent, make_hash, make_hasher};
use crate::raw::{Allocator, Bucket, Global, RawTable};
use crate::{Equivalent, HashMap};
use core::fmt::{self, Debug};
use core::hash::{BuildHasher, Hash};
use core::mem;
#[allow(clippy::missing_safety_doc)]
pub unsafe trait Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()>;
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
}
pub struct RawOccupiedEntryMut<'a, K, V, S, A: Allocator = Global> {
    elem: Bucket<(K, V)>,
    table: &'a mut RawTable<(K, V), A>,
    hash_builder: &'a S,
}
pub struct Bucket<T> {
    ptr: NonNull<T>,
}
pub struct RawTable<T, A: Allocator = Global> {
    table: RawTableInner,
    alloc: A,
    marker: PhantomData<T>,
}
impl<'a, K, V, S, A: Allocator> RawOccupiedEntryMut<'a, K, V, S, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn key(&self) -> &K {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn key_mut(&mut self) -> &mut K {
        unsafe { &mut self.elem.as_mut().0 }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn into_key(self) -> &'a mut K {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get(&self) -> &V {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn into_mut(self) -> &'a mut V {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get_mut(&mut self) -> &mut V {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get_key_value(&self) -> (&K, &V) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get_key_value_mut(&mut self) -> (&mut K, &mut V) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn into_key_value(self) -> (&'a mut K, &'a mut V) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(&mut self, value: V) -> V {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert_key(&mut self, key: K) -> K {
        mem::replace(self.key_mut(), key)
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove(self) -> V {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove_entry(self) -> (K, V) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn replace_entry_with<F>(self, f: F) -> RawEntryMut<'a, K, V, S, A>
    where
        F: FnOnce(&K, V) -> Option<V>,
    {}
}
