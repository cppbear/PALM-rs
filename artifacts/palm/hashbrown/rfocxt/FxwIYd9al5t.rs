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
#[cfg_attr(feature = "inline-more", inline)]
#[allow(dead_code)]
pub(crate) fn equivalent<Q, K>(k: &Q) -> impl Fn(&K) -> bool + '_
where
    Q: Equivalent<K> + ?Sized,
{
    move |x| k.equivalent(x)
}
