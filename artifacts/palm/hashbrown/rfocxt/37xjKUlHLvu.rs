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
pub struct HashMap<K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) hash_builder: S,
    pub(crate) table: RawTable<(K, V), A>,
}
pub struct VacantEntry<'a, K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    hash: u64,
    key: K,
    table: &'a mut HashMap<K, V, S, A>,
}
pub struct OccupiedError<'a, K, V, S, A: Allocator = Global> {
    /// The entry in the map that was already occupied.
    pub entry: OccupiedEntry<'a, K, V, S, A>,
    /// The value which was not inserted, because the entry was already occupied.
    pub value: V,
}
pub struct OccupiedEntry<'a, K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    hash: u64,
    elem: Bucket<(K, V)>,
    table: &'a mut HashMap<K, V, S, A>,
}
pub struct RawTable<T, A: Allocator = Global> {
    table: RawTableInner,
    alloc: A,
    marker: PhantomData<T>,
}
pub enum Entry<'a, K, V, S, A = Global>
where
    A: Allocator,
{
    /// An occupied entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::hash_map::{Entry, HashMap};
    /// let mut map: HashMap<_, _> = [("a", 100), ("b", 200)].into();
    ///
    /// match map.entry("a") {
    ///     Entry::Vacant(_) => unreachable!(),
    ///     Entry::Occupied(_) => { }
    /// }
    /// ```
    Occupied(OccupiedEntry<'a, K, V, S, A>),
    /// A vacant entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashbrown::hash_map::{Entry, HashMap};
    /// let mut map: HashMap<&str, i32> = HashMap::new();
    ///
    /// match map.entry("a") {
    ///     Entry::Occupied(_) => unreachable!(),
    ///     Entry::Vacant(_) => { }
    /// }
    /// ```
    Vacant(VacantEntry<'a, K, V, S, A>),
}
impl<K, V, S, A> HashMap<K, V, S, A>
where
    K: Eq + Hash,
    S: BuildHasher,
    A: Allocator,
{
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn reserve(&mut self, additional: usize) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn shrink_to_fit(&mut self) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn shrink_to(&mut self, min_capacity: usize) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, S, A> {
        let hash = make_hash::<K, S>(&self.hash_builder, &key);
        if let Some(elem) = self.table.find(hash, equivalent_key(&key)) {
            Entry::Occupied(OccupiedEntry {
                hash,
                elem,
                table: self,
            })
        } else {
            Entry::Vacant(VacantEntry {
                hash,
                key,
                table: self,
            })
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn entry_ref<'a, 'b, Q>(
        &'a mut self,
        key: &'b Q,
    ) -> EntryRef<'a, 'b, K, Q, V, S, A>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    #[inline]
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    #[inline]
    pub fn get_key_value<Q>(&self, k: &Q) -> Option<(&K, &V)>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    #[inline]
    fn get_inner<Q>(&self, k: &Q) -> Option<&(K, V)>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    #[inline]
    pub fn get_key_value_mut<Q>(&mut self, k: &Q) -> Option<(&K, &mut V)>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    #[inline]
    fn get_inner_mut<Q>(&mut self, k: &Q) -> Option<&mut (K, V)>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    pub fn get_many_mut<Q, const N: usize>(
        &mut self,
        ks: [&Q; N],
    ) -> [Option<&'_ mut V>; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    pub unsafe fn get_many_unchecked_mut<Q, const N: usize>(
        &mut self,
        ks: [&Q; N],
    ) -> [Option<&'_ mut V>; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    pub fn get_many_key_value_mut<Q, const N: usize>(
        &mut self,
        ks: [&Q; N],
    ) -> [Option<(&'_ K, &'_ mut V)>; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    pub unsafe fn get_many_key_value_unchecked_mut<Q, const N: usize>(
        &mut self,
        ks: [&Q; N],
    ) -> [Option<(&'_ K, &'_ mut V)>; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    fn get_many_mut_inner<Q, const N: usize>(
        &mut self,
        ks: [&Q; N],
    ) -> [Option<&'_ mut (K, V)>; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    unsafe fn get_many_unchecked_mut_inner<Q, const N: usize>(
        &mut self,
        ks: [&Q; N],
    ) -> [Option<&'_ mut (K, V)>; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    fn build_hashes_inner<Q, const N: usize>(&self, ks: [&Q; N]) -> [u64; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub(crate) fn find_or_find_insert_slot<Q>(
        &mut self,
        hash: u64,
        key: &Q,
    ) -> Result<Bucket<(K, V)>, crate::raw::InsertSlot>
    where
        Q: Equivalent<K> + ?Sized,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub unsafe fn insert_unique_unchecked(&mut self, k: K, v: V) -> (&K, &mut V) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn try_insert(
        &mut self,
        key: K,
        value: V,
    ) -> Result<&mut V, OccupiedError<'_, K, V, S, A>> {
        match self.entry(key) {
            Entry::Occupied(entry) => Err(OccupiedError { entry, value }),
            Entry::Vacant(entry) => Ok(entry.insert(value)),
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove_entry<Q>(&mut self, k: &Q) -> Option<(K, V)>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    #[inline]
    pub fn allocation_size(&self) -> usize {}
}
impl<'a, K, V, S, A: Allocator> VacantEntry<'a, K, V, S, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn key(&self) -> &K {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn into_key(self) -> K {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(self, value: V) -> &'a mut V
    where
        K: Hash,
        S: BuildHasher,
    {
        let table = &mut self.table.table;
        let entry = table
            .insert_entry(
                self.hash,
                (self.key, value),
                make_hasher::<_, V, S>(&self.table.hash_builder),
            );
        &mut entry.1
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V, S, A>
    where
        K: Hash,
        S: BuildHasher,
    {}
}
