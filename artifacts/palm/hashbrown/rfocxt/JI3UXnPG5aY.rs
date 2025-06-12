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
pub struct IntoValues<K, V, A: Allocator = Global> {
    inner: IntoIter<K, V, A>,
}
pub struct IntoIter<K, V, A: Allocator = Global> {
    inner: RawIntoIter<(K, V), A>,
}
pub struct Iter<'a, K, V> {
    inner: RawIter<(K, V)>,
    marker: PhantomData<(&'a K, &'a V)>,
}
pub struct IntoIter<T, A = Global>
where
    A: Allocator,
{
    inner: RawIntoIter<T, A>,
}
pub struct IntoIter<K, A: Allocator = Global> {
    iter: map::IntoIter<K, (), A>,
}
impl<K, V: Debug, A: Allocator> fmt::Debug for IntoValues<K, V, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.inner.iter().map(|(_, v)| v)).finish()
    }
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
