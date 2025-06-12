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
pub struct RawVacantEntryMut<'a, K, V, S, A: Allocator = Global> {
    table: &'a mut RawTable<(K, V), A>,
    hash_builder: &'a S,
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
impl<'a, K, V, S, A: Allocator> RawVacantEntryMut<'a, K, V, S, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(self, key: K, value: V) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    #[allow(clippy::shadow_unrelated)]
    pub fn insert_hashed_nocheck(
        self,
        hash: u64,
        key: K,
        value: V,
    ) -> (&'a mut K, &'a mut V)
    where
        K: Hash,
        S: BuildHasher,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert_with_hasher<H>(
        self,
        hash: u64,
        key: K,
        value: V,
        hasher: H,
    ) -> (&'a mut K, &'a mut V)
    where
        H: Fn(&K) -> u64,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    fn insert_entry(self, key: K, value: V) -> RawOccupiedEntryMut<'a, K, V, S, A>
    where
        K: Hash,
        S: BuildHasher,
    {
        let hash = make_hash::<K, S>(self.hash_builder, &key);
        let elem = self
            .table
            .insert(hash, (key, value), make_hasher::<_, V, S>(self.hash_builder));
        RawOccupiedEntryMut {
            elem,
            table: self.table,
            hash_builder: self.hash_builder,
        }
    }
}
#[cfg_attr(feature = "inline-more", inline)]
pub(crate) fn make_hasher<Q, V, S>(hash_builder: &S) -> impl Fn(&(Q, V)) -> u64 + '_
where
    Q: Hash,
    S: BuildHasher,
{
    move |val| make_hash::<Q, S>(hash_builder, &val.0)
}
#[cfg(feature = "nightly")]
#[cfg_attr(feature = "inline-more", inline)]
pub(crate) fn make_hash<Q, S>(hash_builder: &S, val: &Q) -> u64
where
    Q: Hash + ?Sized,
    S: BuildHasher,
{
    hash_builder.hash_one(val)
}
