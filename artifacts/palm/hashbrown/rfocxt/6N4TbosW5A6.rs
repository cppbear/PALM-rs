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
pub struct HashMap<K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) hash_builder: S,
    pub(crate) table: RawTable<(K, V), A>,
}
impl<K, Q, V, S, A> Debug for VacantEntryRef<'_, '_, K, Q, V, S, A>
where
    K: Borrow<Q>,
    Q: Debug + ?Sized,
    A: Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("VacantEntryRef").field(&self.key()).finish()
    }
}
impl<'a, 'b, K, Q: ?Sized, V, S, A: Allocator> VacantEntryRef<'a, 'b, K, Q, V, S, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn key(&self) -> &'b Q {
        self.key
    }
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
    {}
}
