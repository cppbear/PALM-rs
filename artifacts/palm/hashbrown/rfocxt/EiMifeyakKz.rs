use crate::{Equivalent, TryReserveError};
use core::hash::{BuildHasher, Hash};
use core::iter::{Chain, FusedIterator};
use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign,
};
use core::{fmt, mem};
use map::make_hash;
use super::map::{self, HashMap, Keys};
use crate::raw::{Allocator, Global, RawExtractIf};
use crate::DefaultHashBuilder;
#[allow(clippy::missing_safety_doc)]
pub unsafe trait Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()>;
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
}
pub struct HashSet<T, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) map: HashMap<T, (), S, A>,
}
pub struct HashMap<K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) hash_builder: S,
    pub(crate) table: RawTable<(K, V), A>,
}
pub struct Drain<'a, K, V, A: Allocator = Global> {
    inner: RawDrain<'a, (K, V), A>,
}
pub struct Drain<'a, K, A: Allocator = Global> {
    iter: map::Drain<'a, K, (), A>,
}
impl<T, S, A: Allocator> HashSet<T, S, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn capacity(&self) -> usize {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn iter(&self) -> Iter<'_, T> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn len(&self) -> usize {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn is_empty(&self) -> bool {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn drain(&mut self) -> Drain<'_, T, A> {
        Drain { iter: self.map.drain() }
    }
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn extract_if<F>(&mut self, f: F) -> ExtractIf<'_, T, F, A>
    where
        F: FnMut(&T) -> bool,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn clear(&mut self) {}
}
