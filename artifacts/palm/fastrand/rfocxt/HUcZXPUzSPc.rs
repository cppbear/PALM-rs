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
    fn gen_u32(&mut self) -> u32 {}
    #[inline]
    fn gen_u64(&mut self) -> u64 {
        const WY_CONST_0: u64 = 0x2d35_8dcc_aa6c_78a5;
        const WY_CONST_1: u64 = 0x8bb8_4b93_962e_acc9;
        let s = self.0.wrapping_add(WY_CONST_0);
        self.0 = s;
        let t = u128::from(s) * u128::from(s ^ WY_CONST_1);
        (t as u64) ^ (t >> 64) as u64
    }
    #[inline]
    fn gen_u128(&mut self) -> u128 {
        (u128::from(self.gen_u64()) << 64) | u128::from(self.gen_u64())
    }
    #[inline]
    fn gen_mod_u32(&mut self, n: u32) -> u32 {}
    #[inline]
    fn gen_mod_u64(&mut self, n: u64) -> u64 {}
    #[inline]
    fn gen_mod_u128(&mut self, n: u128) -> u128 {}
}
