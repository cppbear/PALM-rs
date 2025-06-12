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
impl<T, S, A> BitAndAssign<&HashSet<T, S, A>> for HashSet<T, S, A>
where
    T: Eq + Hash + Clone,
    S: BuildHasher,
    A: Allocator,
{
    fn bitand_assign(&mut self, rhs: &HashSet<T, S, A>) {
        self.retain(|item| rhs.contains(item));
    }
}
