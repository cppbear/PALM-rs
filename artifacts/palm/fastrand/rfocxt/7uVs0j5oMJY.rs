use core::convert::{TryFrom, TryInto};
use core::ops::{Bound, RangeBounds};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "std")]
pub use global_rng::*;
macro_rules! rng_integer {
    ($t:tt, $unsigned_t:tt, $gen:tt, $mod:tt, $doc:tt) => {
        #[doc = $doc] #[doc = ""] #[doc = " Panics if the range is empty."] #[inline] pub
        fn $t (& mut self, range : impl RangeBounds <$t >) -> $t { let panic_empty_range
        = || { panic!("empty range: {:?}..{:?}", range.start_bound(), range.end_bound())
        }; let low = match range.start_bound() { Bound::Unbounded => core::$t ::MIN,
        Bound::Included(& x) => x, Bound::Excluded(& x) => x.checked_add(1)
        .unwrap_or_else(panic_empty_range), }; let high = match range.end_bound() {
        Bound::Unbounded => core::$t ::MAX, Bound::Included(& x) => x, Bound::Excluded(&
        x) => x.checked_sub(1).unwrap_or_else(panic_empty_range), }; if low > high {
        panic_empty_range(); } if low == core::$t ::MIN && high == core::$t ::MAX { self
        .$gen () as $t } else { let len = high.wrapping_sub(low).wrapping_add(1); low
        .wrapping_add(self.$mod (len as $unsigned_t as _) as $t) } }
    };
}
#[inline]
fn mul_high_u64(a: u64, b: u64) -> u64 {
    (((a as u128) * (b as u128)) >> 64) as u64
}
