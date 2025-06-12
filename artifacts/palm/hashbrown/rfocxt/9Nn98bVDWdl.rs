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
pub struct IterMut<'a, K, V> {
    inner: RawIter<(K, V)>,
    marker: PhantomData<(&'a K, &'a mut V)>,
}
pub struct Iter<'a, K, V> {
    inner: RawIter<(K, V)>,
    marker: PhantomData<(&'a K, &'a V)>,
}
pub struct RawIter<T> {
    pub(crate) iter: RawIterRange<T>,
    items: usize,
}
impl<K, V> IterMut<'_, K, V> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub(super) fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            inner: self.inner.clone(),
            marker: PhantomData,
        }
    }
}
