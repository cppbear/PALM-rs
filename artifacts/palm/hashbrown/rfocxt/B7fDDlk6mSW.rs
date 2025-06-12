use core::{fmt, iter::FusedIterator, marker::PhantomData};
use crate::{
    raw::{
        Allocator, Bucket, Global, InsertSlot, RawDrain, RawExtractIf, RawIntoIter,
        RawIter, RawIterHash, RawTable,
    },
    TryReserveError,
};
pub struct IntoIter<T, A = Global>
where
    A: Allocator,
{
    inner: RawIntoIter<T, A>,
}
pub struct RawIntoIter<T, A: Allocator = Global> {
    iter: RawIter<T>,
    allocation: Option<(NonNull<u8>, Layout, A)>,
    marker: PhantomData<T>,
}
pub struct Iter<'a, T> {
    inner: RawIter<T>,
    marker: PhantomData<&'a T>,
}
pub struct RawIter<T> {
    pub(crate) iter: RawIterRange<T>,
    items: usize,
}
impl<T, A> fmt::Debug for IntoIter<T, A>
where
    T: fmt::Debug,
    A: Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(Iter {
                inner: self.inner.iter(),
                marker: PhantomData,
            })
            .finish()
    }
}
impl<T, A: Allocator> RawIntoIter<T, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn iter(&self) -> RawIter<T> {
        self.iter.clone()
    }
}
