use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
#[allow(clippy::missing_safety_doc)]
pub unsafe trait Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()>;
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
}
pub struct AbsentEntry<'a, T, A = Global>
where
    A: Allocator,
{
    table: &'a mut HashTable<T, A>,
}
pub struct HashTable<T, A = Global>
where
    A: Allocator,
{
    pub(crate) raw: RawTable<T, A>,
}
impl<T: fmt::Debug, A: Allocator> fmt::Debug for AbsentEntry<'_, T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AbsentEntry")
    }
}
