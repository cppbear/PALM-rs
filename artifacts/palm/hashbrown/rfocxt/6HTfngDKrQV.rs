use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
pub struct IterHash<'a, T> {
    inner: RawIterHash<T>,
    marker: PhantomData<&'a T>,
}
pub struct RawIterHash<T> {
    inner: RawIterHashInner,
    _marker: PhantomData<T>,
}
impl<T> fmt::Debug for IterHash<'_, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}
