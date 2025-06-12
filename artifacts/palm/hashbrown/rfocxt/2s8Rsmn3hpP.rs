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
pub struct Intersection<'a, T, S, A: Allocator = Global> {
    iter: Iter<'a, T>,
    other: &'a HashSet<T, S, A>,
}
pub struct Iter<'a, T> {
    inner: RawIter<T>,
    marker: PhantomData<&'a T>,
}
pub struct Iter<'a, K, V> {
    inner: RawIter<(K, V)>,
    marker: PhantomData<(&'a K, &'a V)>,
}
pub struct Iter<'a, K> {
    iter: Keys<'a, K, ()>,
}
pub struct HashSet<T, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) map: HashMap<T, (), S, A>,
}
impl<T, S, A> fmt::Debug for Intersection<'_, T, S, A>
where
    T: fmt::Debug + Eq + Hash,
    S: BuildHasher,
    A: Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}
