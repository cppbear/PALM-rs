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
pub struct IntoIter<K, V, A: Allocator = Global> {
    inner: RawIntoIter<(K, V), A>,
}
pub struct RawIter<T> {
    pub(crate) iter: RawIterRange<T>,
    items: usize,
}
pub struct Iter<'a, K, V> {
    inner: RawIter<(K, V)>,
    marker: PhantomData<(&'a K, &'a V)>,
}
pub struct RawIntoIter<T, A: Allocator = Global> {
    iter: RawIter<T>,
    allocation: Option<(NonNull<u8>, Layout, A)>,
    marker: PhantomData<T>,
}
impl<K, V, A: Allocator> IntoIter<K, V, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub(super) fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            inner: self.inner.iter(),
            marker: PhantomData,
        }
    }
}
