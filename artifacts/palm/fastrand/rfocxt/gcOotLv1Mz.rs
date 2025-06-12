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
#[derive(Debug, PartialEq, Eq)]
pub struct Rng(u64);
impl Rng {
    #[inline]
    fn gen_u32(&mut self) -> u32 {
        self.gen_u64() as u32
    }
    #[inline]
    fn gen_u64(&mut self) -> u64 {}
    #[inline]
    fn gen_u128(&mut self) -> u128 {}
    #[inline]
    fn gen_mod_u32(&mut self, n: u32) -> u32 {
        let mut r = self.gen_u32();
        let mut hi = mul_high_u32(r, n);
        let mut lo = r.wrapping_mul(n);
        if lo < n {
            let t = n.wrapping_neg() % n;
            while lo < t {
                r = self.gen_u32();
                hi = mul_high_u32(r, n);
                lo = r.wrapping_mul(n);
            }
        }
        hi
    }
    #[inline]
    fn gen_mod_u64(&mut self, n: u64) -> u64 {}
    #[inline]
    fn gen_mod_u128(&mut self, n: u128) -> u128 {}
}
#[inline]
fn mul_high_u32(a: u32, b: u32) -> u32 {
    (((a as u64) * (b as u64)) >> 32) as u32
}
