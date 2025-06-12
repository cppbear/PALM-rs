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
pub struct Drain<'a, K, A: Allocator = Global> {
    iter: map::Drain<'a, K, (), A>,
}
pub struct Iter<'a, K, V> {
    inner: RawIter<(K, V)>,
    marker: PhantomData<(&'a K, &'a V)>,
}
pub struct Drain<'a, K, V, A: Allocator = Global> {
    inner: RawDrain<'a, (K, V), A>,
}
pub struct Drain<'a, T, A: Allocator = Global> {
    inner: RawDrain<'a, T, A>,
}
impl<K: fmt::Debug, A: Allocator> fmt::Debug for Drain<'_, K, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entries_iter = self.iter.iter().map(|(k, _)| k);
        f.debug_list().entries(entries_iter).finish()
    }
}
