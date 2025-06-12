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
pub struct RawEntryBuilder<'a, K, V, S, A: Allocator = Global> {
    map: &'a HashMap<K, V, S, A>,
}
pub struct HashMap<K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) hash_builder: S,
    pub(crate) table: RawTable<(K, V), A>,
}
pub struct RawTable<T, A: Allocator = Global> {
    table: RawTableInner,
    alloc: A,
    marker: PhantomData<T>,
}
impl<'a, K, V, S, A: Allocator> RawEntryBuilder<'a, K, V, S, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    #[allow(clippy::wrong_self_convention)]
    pub fn from_key<Q>(self, k: &Q) -> Option<(&'a K, &'a V)>
    where
        S: BuildHasher,
        Q: Hash + Equivalent<K> + ?Sized,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    #[allow(clippy::wrong_self_convention)]
    pub fn from_key_hashed_nocheck<Q>(self, hash: u64, k: &Q) -> Option<(&'a K, &'a V)>
    where
        Q: Equivalent<K> + ?Sized,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    fn search<F>(self, hash: u64, mut is_match: F) -> Option<(&'a K, &'a V)>
    where
        F: FnMut(&K) -> bool,
    {
        match self.map.table.get(hash, |(k, _)| is_match(k)) {
            Some((key, value)) => Some((key, value)),
            None => None,
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    #[allow(clippy::wrong_self_convention)]
    pub fn from_hash<F>(self, hash: u64, is_match: F) -> Option<(&'a K, &'a V)>
    where
        F: FnMut(&K) -> bool,
    {}
}
