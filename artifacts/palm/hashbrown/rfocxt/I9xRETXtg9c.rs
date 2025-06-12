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
pub struct Drain<'a, T, A: Allocator = Global> {
    inner: RawDrain<'a, T, A>,
}
pub struct RawDrain<'a, T, A: Allocator = Global> {
    iter: RawIter<T>,
    table: RawTableInner,
    orig_table: NonNull<RawTableInner>,
    marker: PhantomData<&'a RawTable<T, A>>,
}
pub struct Iter<'a, T> {
    inner: RawIter<T>,
    marker: PhantomData<&'a T>,
}
pub struct RawIter<T> {
    pub(crate) iter: RawIterRange<T>,
    items: usize,
}
impl<T: fmt::Debug, A: Allocator> fmt::Debug for Drain<'_, T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(Iter {
                inner: self.inner.iter(),
                marker: PhantomData,
            })
            .finish()
    }
}
impl<T, A: Allocator> RawDrain<'_, T, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn iter(&self) -> RawIter<T> {
        self.iter.clone()
    }
}
