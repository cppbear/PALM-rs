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
fn mul_high_u128(a: u128, b: u128) -> u128 {
    let a_lo = a as u64 as u128;
    let a_hi = (a >> 64) as u64 as u128;
    let b_lo = b as u64 as u128;
    let b_hi = (b >> 64) as u64 as u128;
    let carry = (a_lo * b_lo) >> 64;
    let carry = ((a_hi * b_lo) as u64 as u128 + (a_lo * b_hi) as u64 as u128 + carry)
        >> 64;
    a_hi * b_hi + ((a_hi * b_lo) >> 64) + ((a_lo * b_hi) >> 64) + carry
}
