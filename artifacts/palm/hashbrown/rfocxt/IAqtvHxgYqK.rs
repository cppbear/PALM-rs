use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
pub struct OccupiedEntry<'a, T, A = Global>
where
    A: Allocator,
{
    hash: u64,
    bucket: Bucket<T>,
    table: &'a mut HashTable<T, A>,
}
pub struct OccupiedEntry<'a, T, S, A: Allocator = Global> {
    inner: map::OccupiedEntry<'a, T, (), S, A>,
}
pub struct VacantEntry<'a, T, A = Global>
where
    A: Allocator,
{
    hash: u64,
    insert_slot: InsertSlot,
    table: &'a mut HashTable<T, A>,
}
pub struct VacantEntry<'a, T, S, A: Allocator = Global> {
    inner: map::VacantEntry<'a, T, (), S, A>,
}
pub struct VacantEntry<'a, K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    hash: u64,
    key: K,
    table: &'a mut HashMap<K, V, S, A>,
}
pub struct OccupiedEntry<'a, K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    hash: u64,
    elem: Bucket<(K, V)>,
    table: &'a mut HashMap<K, V, S, A>,
}
pub enum Entry<'a, T, A = Global>
where
    A: Allocator,
{
    /// An occupied entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "nightly")]
    /// # fn test() {
    /// use hashbrown::hash_table::{Entry, OccupiedEntry};
    /// use hashbrown::{HashTable, DefaultHashBuilder};
    /// use std::hash::BuildHasher;
    ///
    /// let mut table = HashTable::new();
    /// let hasher = DefaultHashBuilder::default();
    /// let hasher = |val: &_| hasher.hash_one(val);
    /// for x in ["a", "b"] {
    ///     table.insert_unique(hasher(&x), x, hasher);
    /// }
    ///
    /// match table.entry(hasher(&"a"), |&x| x == "a", hasher) {
    ///     Entry::Vacant(_) => unreachable!(),
    ///     Entry::Occupied(_) => {}
    /// }
    /// # }
    /// # fn main() {
    /// #     #[cfg(feature = "nightly")]
    /// #     test()
    /// # }
    /// ```
    Occupied(OccupiedEntry<'a, T, A>),
    /// A vacant entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "nightly")]
    /// # fn test() {
    /// use hashbrown::hash_table::{Entry, OccupiedEntry};
    /// use hashbrown::{HashTable, DefaultHashBuilder};
    /// use std::hash::BuildHasher;
    ///
    /// let mut table = HashTable::<&str>::new();
    /// let hasher = DefaultHashBuilder::default();
    /// let hasher = |val: &_| hasher.hash_one(val);
    ///
    /// match table.entry(hasher(&"a"), |&x| x == "a", hasher) {
    ///     Entry::Vacant(_) => {}
    ///     Entry::Occupied(_) => unreachable!(),
    /// }
    /// # }
    /// # fn main() {
    /// #     #[cfg(feature = "nightly")]
    /// #     test()
    /// # }
    /// ```
    Vacant(VacantEntry<'a, T, A>),
}
impl<'a, T, A> Entry<'a, T, A>
where
    A: Allocator,
{
    pub fn insert(self, value: T) -> OccupiedEntry<'a, T, A> {}
    pub fn or_insert(self, default: T) -> OccupiedEntry<'a, T, A> {}
    pub fn or_insert_with(self, default: impl FnOnce() -> T) -> OccupiedEntry<'a, T, A> {}
    pub fn and_modify(self, f: impl FnOnce(&mut T)) -> Self {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}
impl<'a, T, A> OccupiedEntry<'a, T, A>
where
    A: Allocator,
{
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove(self) -> (T, VacantEntry<'a, T, A>) {}
    #[inline]
    pub fn get(&self) -> &T {}
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { self.bucket.as_mut() }
    }
    pub fn into_mut(self) -> &'a mut T {}
    pub fn into_table(self) -> &'a mut HashTable<T, A> {}
}
