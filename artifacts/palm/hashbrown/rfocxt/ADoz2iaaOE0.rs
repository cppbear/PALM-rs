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
pub struct VacantEntry<'a, T, S, A: Allocator = Global> {
    inner: map::VacantEntry<'a, T, (), S, A>,
}
pub struct VacantEntry<'a, K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    hash: u64,
    key: K,
    table: &'a mut HashMap<K, V, S, A>,
}
pub struct VacantEntry<'a, T, A = Global>
where
    A: Allocator,
{
    hash: u64,
    insert_slot: InsertSlot,
    table: &'a mut HashTable<T, A>,
}
impl<'a, T, S, A: Allocator> VacantEntry<'a, T, S, A> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn get(&self) -> &T {}
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn into_value(self) -> T {
        self.inner.into_key()
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(self) -> OccupiedEntry<'a, T, S, A>
    where
        T: Hash,
        S: BuildHasher,
    {}
}
