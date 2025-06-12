use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
pub struct IterMut<'a, T> {
    inner: RawIter<T>,
    marker: PhantomData<&'a mut T>,
}
pub struct Iter<'a, T> {
    inner: RawIter<T>,
    marker: PhantomData<&'a T>,
}
pub struct RawIter<T> {
    pub(crate) iter: RawIterRange<T>,
    items: usize,
}
impl<T> fmt::Debug for IterMut<'_, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(Iter {
                inner: self.inner.clone(),
                marker: PhantomData,
            })
            .finish()
    }
}
