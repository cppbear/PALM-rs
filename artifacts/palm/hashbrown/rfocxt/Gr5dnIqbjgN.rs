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
pub struct RawEntryBuilderMut<'a, K, V, S, A: Allocator = Global> {
    map: &'a mut HashMap<K, V, S, A>,
}
pub struct HashMap<K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) hash_builder: S,
    pub(crate) table: RawTable<(K, V), A>,
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
impl<'a, K, V, S, A: Allocator> RawEntryBuilderMut<'a, K, V, S, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    #[allow(clippy::wrong_self_convention)]
    pub fn from_key<Q>(self, k: &Q) -> RawEntryMut<'a, K, V, S, A>
    where
        S: BuildHasher,
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub fn from_key_hashed_nocheck<Q>(
        self,
        hash: u64,
        k: &Q,
    ) -> RawEntryMut<'a, K, V, S, A>
    where
        Q: Equivalent<K> + ?Sized,
    {
        self.from_hash(hash, equivalent(k))
    }
}
#[cfg_attr(feature = "inline-more", inline)]
#[allow(dead_code)]
pub(crate) fn equivalent<Q, K>(k: &Q) -> impl Fn(&K) -> bool + '_
where
    Q: Equivalent<K> + ?Sized,
{
    move |x| k.equivalent(x)
}
