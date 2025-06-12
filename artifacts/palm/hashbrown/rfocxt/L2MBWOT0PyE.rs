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
pub struct VacantEntryRef<'a, 'b, K, Q: ?Sized, V, S, A: Allocator = Global> {
    hash: u64,
    key: &'b Q,
    table: &'a mut HashMap<K, V, S, A>,
}
pub struct RawTable<T, A: Allocator = Global> {
    table: RawTableInner,
    alloc: A,
    marker: PhantomData<T>,
}
pub struct OccupiedEntry<'a, K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    hash: u64,
    elem: Bucket<(K, V)>,
    table: &'a mut HashMap<K, V, S, A>,
}
pub struct HashMap<K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) hash_builder: S,
    pub(crate) table: RawTable<(K, V), A>,
}
pub struct Bucket<T> {
    ptr: NonNull<T>,
}
impl<'a, 'b, K, Q: ?Sized, V, S, A: Allocator> VacantEntryRef<'a, 'b, K, Q, V, S, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn key(&self) -> &'b Q {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(self, value: V) -> &'a mut V
    where
        K: Hash + From<&'b Q>,
        S: BuildHasher,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V, S, A>
    where
        K: Hash + From<&'b Q>,
        S: BuildHasher,
    {
        let elem = self
            .table
            .table
            .insert(
                self.hash,
                (self.key.into(), value),
                make_hasher::<_, V, S>(&self.table.hash_builder),
            );
        OccupiedEntry {
            hash: self.hash,
            elem,
            table: self.table,
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
