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
impl<K, V, S, A: Allocator> Debug for RawEntryBuilderMut<'_, K, V, S, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawEntryBuilder").finish()
    }
}
