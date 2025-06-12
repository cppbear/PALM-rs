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
pub struct Bucket<T> {
    ptr: NonNull<T>,
}
pub struct InsertSlot {
    index: usize,
}
pub struct Iter<'a, K> {
    iter: Keys<'a, K, ()>,
}
pub struct RawTable<T, A: Allocator = Global> {
    table: RawTableInner,
    alloc: A,
    marker: PhantomData<T>,
}
impl<T, S, A> BitXorAssign<&HashSet<T, S, A>> for HashSet<T, S, A>
where
    T: Eq + Hash + Clone,
    S: BuildHasher,
    A: Allocator,
{
    fn bitxor_assign(&mut self, rhs: &HashSet<T, S, A>) {
        for item in rhs {
            let hash = make_hash(&self.map.hash_builder, item);
            match self.map.find_or_find_insert_slot(hash, item) {
                Ok(bucket) => {
                    unsafe {
                        self.map.table.remove(bucket);
                    }
                }
                Err(slot) => {
                    unsafe {
                        self.map.table.insert_in_slot(hash, slot, (item.clone(), ()));
                    }
                }
            }
        }
    }
}
#[cfg(feature = "nightly")]
#[cfg_attr(feature = "inline-more", inline)]
pub(crate) fn make_hash<Q, S>(hash_builder: &S, val: &Q) -> u64
where
    Q: Hash + ?Sized,
    S: BuildHasher,
{
    hash_builder.hash_one(val)
}
