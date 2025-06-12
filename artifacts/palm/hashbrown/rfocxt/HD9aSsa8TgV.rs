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
impl<T, S, A> HashSet<T, S, A>
where
    T: Eq + Hash,
    S: BuildHasher,
    A: Allocator,
{
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn reserve(&mut self, additional: usize) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn shrink_to_fit(&mut self) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn shrink_to(&mut self, min_capacity: usize) {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, T, S, A> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn symmetric_difference<'a>(
        &'a self,
        other: &'a Self,
    ) -> SymmetricDifference<'a, T, S, A> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a, T, S, A> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn union<'a>(&'a self, other: &'a Self) -> Union<'a, T, S, A> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        Q: Hash + Equivalent<T> + ?Sized,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        Q: Hash + Equivalent<T> + ?Sized,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get_or_insert(&mut self, value: T) -> &T {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get_or_insert_with<Q, F>(&mut self, value: &Q, f: F) -> &T
    where
        Q: Hash + Equivalent<T> + ?Sized,
        F: FnOnce(&Q) -> T,
    {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn entry(&mut self, value: T) -> Entry<'_, T, S, A> {}
    pub fn is_disjoint(&self, other: &Self) -> bool {}
    pub fn is_subset(&self, other: &Self) -> bool {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn is_superset(&self, other: &Self) -> bool {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(&mut self, value: T) -> bool {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub unsafe fn insert_unique_unchecked(&mut self, value: T) -> &T {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn replace(&mut self, value: T) -> Option<T> {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: Hash + Equivalent<T> + ?Sized,
    {
        self.map.remove(value).is_some()
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn take<Q>(&mut self, value: &Q) -> Option<T>
    where
        Q: Hash + Equivalent<T> + ?Sized,
    {}
    #[inline]
    pub fn allocation_size(&self) -> usize {}
}
