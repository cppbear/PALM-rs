use core::fmt;
use core::mem::MaybeUninit;
use core::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo,
    RangeToInclusive,
};
macro_rules! impl_index {
    ($($t:ty),*) => {
        $(impl Index <$t > for UninitSlice { type Output = UninitSlice; #[inline] fn
        index(& self, index : $t) -> & UninitSlice { UninitSlice::uninit_ref(& self
        .0[index]) } } impl IndexMut <$t > for UninitSlice { #[inline] fn index_mut(& mut
        self, index : $t) -> & mut UninitSlice { UninitSlice::uninit(& mut self.0[index])
        } })*
    };
}
impl_index!(
    Range < usize >, RangeFrom < usize >, RangeFull, RangeInclusive < usize >, RangeTo <
    usize >, RangeToInclusive < usize >
);
#[repr(transparent)]
pub struct UninitSlice([MaybeUninit<u8>]);
impl fmt::Debug for UninitSlice {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("UninitSlice[...]").finish()
    }
}
