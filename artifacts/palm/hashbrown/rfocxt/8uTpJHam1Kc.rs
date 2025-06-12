use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
pub struct IterHashMut<'a, T> {
    inner: RawIterHash<T>,
    marker: PhantomData<&'a mut T>,
}
pub struct RawIterHash<T> {
    inner: RawIterHashInner,
    _marker: PhantomData<T>,
}
pub struct IterHash<'a, T> {
    inner: RawIterHash<T>,
    marker: PhantomData<&'a T>,
}
impl<T> fmt::Debug for IterHashMut<'_, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(IterHash {
                inner: self.inner.clone(),
                marker: PhantomData,
            })
            .finish()
    }
}
