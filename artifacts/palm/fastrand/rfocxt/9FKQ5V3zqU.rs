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
    #[must_use = "this creates a new instance of `Rng`; if you want to initialize the thread-local generator, use `fastrand::seed()` instead"]
    pub fn with_seed(seed: u64) -> Self {
        Rng(seed)
    }
    #[inline]
    #[must_use = "this creates a new instance of `Rng`"]
    pub fn fork(&mut self) -> Self {
        Rng::with_seed(self.gen_u64())
    }
    #[inline]
    pub fn alphabetic(&mut self) -> char {}
    #[inline]
    pub fn alphanumeric(&mut self) -> char {}
    #[inline]
    pub fn bool(&mut self) -> bool {
        self.u8(..) % 2 == 0
    }
    #[inline]
    pub fn digit(&mut self, base: u32) -> char {
        if base == 0 {
            panic!("base cannot be zero");
        }
        if base > 36 {
            panic!("base cannot be larger than 36");
        }
        let num = self.u8(..base as u8);
        if num < 10 { (b'0' + num) as char } else { (b'a' + num - 10) as char }
    }
    pub fn f32(&mut self) -> f32 {
        let b = 32;
        let f = core::f32::MANTISSA_DIGITS - 1;
        f32::from_bits((1 << (b - 2)) - (1 << f) + (self.u32(..) >> (b - f))) - 1.0
    }
    pub fn f64(&mut self) -> f64 {
        let b = 64;
        let f = core::f64::MANTISSA_DIGITS - 1;
        f64::from_bits((1 << (b - 2)) - (1 << f) + (self.u64(..) >> (b - f))) - 1.0
    }
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn choose_multiple<I: IntoIterator>(
        &mut self,
        source: I,
        amount: usize,
    ) -> Vec<I::Item> {}
    #[inline]
    pub fn lowercase(&mut self) -> char {}
    #[inline]
    pub fn seed(&mut self, seed: u64) {}
    #[inline]
    pub fn get_seed(&self) -> u64 {}
    #[inline]
    pub fn choice<I>(&mut self, iter: I) -> Option<I::Item>
    where
        I: IntoIterator,
        I::IntoIter: ExactSizeIterator,
    {}
    #[inline]
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {}
    #[inline]
    pub fn fill(&mut self, slice: &mut [u8]) {}
    #[inline]
    pub fn uppercase(&mut self) -> char {}
    #[inline]
    pub fn char(&mut self, range: impl RangeBounds<char>) -> char {
        let panic_empty_range = || {
            panic!("empty range: {:?}..{:?}", range.start_bound(), range.end_bound())
        };
        let surrogate_start = 0xd800u32;
        let surrogate_len = 0x800u32;
        let low = match range.start_bound() {
            Bound::Unbounded => 0u8 as char,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => {
                let scalar = if x as u32 == surrogate_start - 1 {
                    surrogate_start + surrogate_len
                } else {
                    x as u32 + 1
                };
                char::try_from(scalar).unwrap_or_else(|_| panic_empty_range())
            }
        };
        let high = match range.end_bound() {
            Bound::Unbounded => core::char::MAX,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => {
                let scalar = if x as u32 == surrogate_start + surrogate_len {
                    surrogate_start - 1
                } else {
                    (x as u32).wrapping_sub(1)
                };
                char::try_from(scalar).unwrap_or_else(|_| panic_empty_range())
            }
        };
        if low > high {
            panic_empty_range();
        }
        let gap = if (low as u32) < surrogate_start && (high as u32) >= surrogate_start {
            surrogate_len
        } else {
            0
        };
        let range = high as u32 - low as u32 - gap;
        let mut val = self.u32(0..=range) + low as u32;
        if val >= surrogate_start {
            val += gap;
        }
        val.try_into().unwrap()
    }
}
