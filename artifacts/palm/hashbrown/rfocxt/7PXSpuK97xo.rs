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
impl<'a, K, V, S, A: Allocator> RawEntryMut<'a, K, V, S, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(self, key: K, value: V) -> RawOccupiedEntryMut<'a, K, V, S, A>
    where
        K: Hash,
        S: BuildHasher,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn or_insert(self, default_key: K, default_val: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn or_insert_with<F>(self, default: F) -> (&'a mut K, &'a mut V)
    where
        F: FnOnce() -> (K, V),
        K: Hash,
        S: BuildHasher,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut K, &mut V),
    {
        match self {
            RawEntryMut::Occupied(mut entry) => {
                {
                    let (k, v) = entry.get_key_value_mut();
                    f(k, v);
                }
                RawEntryMut::Occupied(entry)
            }
            RawEntryMut::Vacant(entry) => RawEntryMut::Vacant(entry),
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn and_replace_entry_with<F>(self, f: F) -> Self
    where
        F: FnOnce(&K, V) -> Option<V>,
    {
        match self {
            RawEntryMut::Occupied(entry) => entry.replace_entry_with(f),
            RawEntryMut::Vacant(_) => self,
        }
    }
}
