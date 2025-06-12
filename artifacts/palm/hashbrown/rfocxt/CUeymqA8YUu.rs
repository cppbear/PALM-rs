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
pub struct RawTable<T, A: Allocator = Global> {
    table: RawTableInner,
    alloc: A,
    marker: PhantomData<T>,
}
pub struct Bucket<T> {
    ptr: NonNull<T>,
}
pub struct RawVacantEntryMut<'a, K, V, S, A: Allocator = Global> {
    table: &'a mut RawTable<(K, V), A>,
    hash_builder: &'a S,
}
pub enum RawEntryMut<'a, K, V, S, A: Allocator = Global> {
    /// An occupied entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::{hash_map::RawEntryMut, HashMap};
    /// let mut map: HashMap<_, _> = [("a", 100), ("b", 200)].into();
    ///
    /// match map.raw_entry_mut().from_key(&"a") {
    ///     RawEntryMut::Vacant(_) => unreachable!(),
    ///     RawEntryMut::Occupied(_) => { }
    /// }
    /// ```
    Occupied(RawOccupiedEntryMut<'a, K, V, S, A>),
    /// A vacant entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::{hash_map::RawEntryMut, HashMap};
    /// let mut map: HashMap<&str, i32> = HashMap::new();
    ///
    /// match map.raw_entry_mut().from_key("a") {
    ///     RawEntryMut::Occupied(_) => unreachable!(),
    ///     RawEntryMut::Vacant(_) => { }
    /// }
    /// ```
    Vacant(RawVacantEntryMut<'a, K, V, S, A>),
}
impl<'a, K, V, S, A: Allocator> RawOccupiedEntryMut<'a, K, V, S, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn key(&self) -> &K {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn key_mut(&mut self) -> &mut K {}
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
    pub fn insert_key(&mut self, key: K) -> K {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove(self) -> V {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove_entry(self) -> (K, V) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn replace_entry_with<F>(self, f: F) -> RawEntryMut<'a, K, V, S, A>
    where
        F: FnOnce(&K, V) -> Option<V>,
    {
        unsafe {
            let still_occupied = self
                .table
                .replace_bucket_with(
                    self.elem.clone(),
                    |(key, value)| { f(&key, value).map(|new_value| (key, new_value)) },
                );
            if still_occupied {
                RawEntryMut::Occupied(self)
            } else {
                RawEntryMut::Vacant(RawVacantEntryMut {
                    table: self.table,
                    hash_builder: self.hash_builder,
                })
            }
        }
    }
}
