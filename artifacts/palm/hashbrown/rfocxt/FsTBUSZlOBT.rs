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
pub struct SymmetricDifference<'a, T, S, A: Allocator = Global> {
    iter: Chain<Difference<'a, T, S, A>, Difference<'a, T, S, A>>,
}
pub struct Difference<'a, T, S, A: Allocator = Global> {
    iter: Iter<'a, T>,
    other: &'a HashSet<T, S, A>,
}
impl<T, S, A> fmt::Debug for SymmetricDifference<'_, T, S, A>
where
    T: fmt::Debug + Eq + Hash,
    S: BuildHasher,
    A: Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}
