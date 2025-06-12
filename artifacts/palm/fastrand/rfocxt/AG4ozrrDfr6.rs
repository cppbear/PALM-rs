use crate::Rng;
use std::cell::Cell;
use std::ops::RangeBounds;
use std::vec::Vec;
const DEFAULT_RNG_SEED: u64 = 0xef6f79ed30ba75a;
std::thread_local! {
    static RNG : Cell < Rng > = Cell::new(Rng(random_seed()
    .unwrap_or(DEFAULT_RNG_SEED)));
}
macro_rules! integer {
    ($t:tt, $doc:tt) => {
        #[doc = $doc] #[doc = ""] #[doc = " Panics if the range is empty."] #[inline] pub
        fn $t (range : impl RangeBounds <$t >) -> $t { with_rng(| r | r.$t (range)) }
    };
}
integer!(u8, "Generates a random `u8` in the given range.");
integer!(i8, "Generates a random `i8` in the given range.");
integer!(u16, "Generates a random `u16` in the given range.");
integer!(i16, "Generates a random `i16` in the given range.");
integer!(u32, "Generates a random `u32` in the given range.");
integer!(i32, "Generates a random `i32` in the given range.");
integer!(u64, "Generates a random `u64` in the given range.");
integer!(i64, "Generates a random `i64` in the given range.");
integer!(u128, "Generates a random `u128` in the given range.");
integer!(i128, "Generates a random `i128` in the given range.");
integer!(usize, "Generates a random `usize` in the given range.");
integer!(isize, "Generates a random `isize` in the given range.");
integer!(char, "Generates a random `char` in the given range.");
#[inline]
pub fn digit(base: u32) -> char {
    with_rng(|r| r.digit(base))
}
#[inline]
fn with_rng<R>(f: impl FnOnce(&mut Rng) -> R) -> R {
    RNG.with(|rng| {
        let current = rng.replace(Rng(0));
        let mut restore = RestoreOnDrop { rng, current };
        f(&mut restore.current)
    })
}
