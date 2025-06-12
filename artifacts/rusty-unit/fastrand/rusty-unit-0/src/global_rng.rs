//! A global, thread-local random number generator.

use crate::Rng;

use std::cell::Cell;
use std::ops::RangeBounds;
use std::vec::Vec;

// Chosen by fair roll of the dice.
const DEFAULT_RNG_SEED: u64 = 0xef6f79ed30ba75a;

impl Default for Rng {
    /// Initialize the `Rng` from the system's random number generator.
    ///
    /// This is equivalent to [`Rng::new()`].
    #[inline]
    fn default() -> Rng {
        Rng::new()
    }
}

impl Rng {
    /// Creates a new random number generator.
    #[inline]
    pub fn new() -> Rng {
        try_with_rng(Rng::fork).unwrap_or_else(|_| Rng::with_seed(0x4d595df4d0f33173))
    }
}

std::thread_local! {
    static RNG: Cell<Rng> = Cell::new(Rng(random_seed().unwrap_or(DEFAULT_RNG_SEED)));
}

/// Run an operation with the current thread-local generator.
#[inline]
fn with_rng<R>(f: impl FnOnce(&mut Rng) -> R) -> R {
    RNG.with(|rng| {
        let current = rng.replace(Rng(0));

        let mut restore = RestoreOnDrop { rng, current };

        f(&mut restore.current)
    })
}

/// Try to run an operation with the current thread-local generator.
#[inline]
fn try_with_rng<R>(f: impl FnOnce(&mut Rng) -> R) -> Result<R, std::thread::AccessError> {
    RNG.try_with(|rng| {
        let current = rng.replace(Rng(0));

        let mut restore = RestoreOnDrop { rng, current };

        f(&mut restore.current)
    })
}

/// Make sure the original RNG is restored even on panic.
struct RestoreOnDrop<'a> {
    rng: &'a Cell<Rng>,
    current: Rng,
}

impl Drop for RestoreOnDrop<'_> {
    fn drop(&mut self) {
        self.rng.set(Rng(self.current.0));
    }
}

/// Initializes the thread-local generator with the given seed.
#[inline]
pub fn seed(seed: u64) {
    with_rng(|r| r.seed(seed));
}

/// Gives back **current** seed that is being held by the thread-local generator.
#[inline]
pub fn get_seed() -> u64 {
    with_rng(|r| r.get_seed())
}

/// Generates a random `bool`.
#[inline]
pub fn bool() -> bool {
    with_rng(|r| r.bool())
}

/// Generates a random `char` in ranges a-z and A-Z.
#[inline]
pub fn alphabetic() -> char {
    with_rng(|r| r.alphabetic())
}

/// Generates a random `char` in ranges a-z, A-Z and 0-9.
#[inline]
pub fn alphanumeric() -> char {
    with_rng(|r| r.alphanumeric())
}

/// Generates a random `char` in range a-z.
#[inline]
pub fn lowercase() -> char {
    with_rng(|r| r.lowercase())
}

/// Generates a random `char` in range A-Z.
#[inline]
pub fn uppercase() -> char {
    with_rng(|r| r.uppercase())
}

/// Choose an item from an iterator at random.
///
/// This function may have an unexpected result if the `len()` property of the
/// iterator does not match the actual number of items in the iterator. If
/// the iterator is empty, this returns `None`.
#[inline]
pub fn choice<I>(iter: I) -> Option<I::Item>
where
    I: IntoIterator,
    I::IntoIter: ExactSizeIterator,
{
    with_rng(|r| r.choice(iter))
}

/// Generates a random digit in the given `base`.
///
/// Digits are represented by `char`s in ranges 0-9 and a-z.
///
/// Panics if the base is zero or greater than 36.
#[inline]
pub fn digit(base: u32) -> char {
    with_rng(|r| r.digit(base))
}

/// Shuffles a slice randomly.
#[inline]
pub fn shuffle<T>(slice: &mut [T]) {
    with_rng(|r| r.shuffle(slice))
}

/// Fill a byte slice with random data.
#[inline]
pub fn fill(slice: &mut [u8]) {
    with_rng(|r| r.fill(slice))
}

macro_rules! integer {
    ($t:tt, $doc:tt) => {
        #[doc = $doc]
        ///
        /// Panics if the range is empty.
        #[inline]
        pub fn $t(range: impl RangeBounds<$t>) -> $t {
            with_rng(|r| r.$t(range))
        }
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

/// Generates a random `f32` in range `0..1`.
pub fn f32() -> f32 {
    with_rng(|r| r.f32())
}

/// Generates a random `f64` in range `0..1`.
pub fn f64() -> f64 {
    with_rng(|r| r.f64())
}

/// Collects `amount` values at random from the iterable into a vector.
pub fn choose_multiple<I: IntoIterator>(source: I, amount: usize) -> Vec<I::Item> {
    with_rng(|rng| rng.choose_multiple(source, amount))
}

#[cfg(not(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
)))]
fn random_seed() -> Option<u64> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::thread;
    use std::time::Instant;

    let mut hasher = DefaultHasher::new();
    Instant::now().hash(&mut hasher);
    thread::current().id().hash(&mut hasher);
    Some(hasher.finish())
}

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown",
    feature = "js"
))]
fn random_seed() -> Option<u64> {
    // TODO(notgull): Failures should be logged somewhere.
    let mut seed = [0u8; 8];
    getrandom::getrandom(&mut seed).ok()?;
    Some(u64::from_ne_bytes(seed))
}

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown",
    not(feature = "js")
))]
fn random_seed() -> Option<u64> {
    None
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2() {
    rusty_monitor::set_test_id(2);
    let mut u64_0: u64 = 2579u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_0_ref_0: &crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &crate::Rng = &mut rng_1;
    let mut u64_1: u64 = 3459u64;
    let mut rng_2: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u64_2: u64 = 9691u64;
    let mut rng_3: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::new();
    let mut rng_5: crate::Rng = crate::Rng::new();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::default();
    let mut rng_6_ref_0: &crate::Rng = &mut rng_6;
    let mut rng_7: crate::Rng = crate::Rng::new();
    let mut rng_7_ref_0: &crate::Rng = &mut rng_7;
    let mut bool_0: bool = crate::Rng::eq(rng_7_ref_0, rng_6_ref_0);
    let mut rng_8: crate::Rng = crate::Rng::fork(rng_5_ref_0);
    let mut char_0: char = crate::Rng::uppercase(rng_3_ref_0);
    let mut rng_8_ref_0: &mut crate::Rng = &mut rng_8;
    let mut char_1: char = crate::Rng::alphanumeric(rng_8_ref_0);
    let mut char_2: char = crate::global_rng::uppercase();
    let mut rng_4_ref_0: &crate::Rng = &mut rng_4;
    let mut u64_3: u64 = crate::Rng::get_seed(rng_4_ref_0);
    let mut char_3: char = crate::global_rng::alphanumeric();
    let mut u64_4: u64 = crate::Rng::get_seed(rng_1_ref_0);
    let mut u64_5: u64 = crate::Rng::get_seed(rng_0_ref_0);
    let mut f32_0: f32 = crate::global_rng::f32();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut u64_0: u64 = 785u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u64_1: u64 = 2691u64;
    let mut rng_2: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::new();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u64_2: u64 = 8331u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u64_3: u64 = 4677u64;
    let mut u64_4: u64 = 235u64;
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_4);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::default();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut rng_7: crate::Rng = crate::Rng::default();
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut u64_5: u64 = 1306u64;
    let mut rng_8: crate::Rng = crate::Rng::with_seed(u64_5);
    let mut rng_8_ref_0: &crate::Rng = &mut rng_8;
    let mut f64_0: f64 = crate::Rng::f64(rng_7_ref_0);
    let mut u64_6: u64 = crate::global_rng::get_seed();
    let mut f32_0: f32 = crate::Rng::f32(rng_5_ref_0);
    crate::global_rng::seed(u64_3);
    let mut char_0: char = crate::Rng::alphanumeric(rng_4_ref_0);
    let mut rng_9: crate::Rng = crate::Rng::default();
    let mut rng_9_ref_0: &crate::Rng = &mut rng_9;
    let mut u64_7: u64 = crate::Rng::get_seed(rng_9_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6() {
    rusty_monitor::set_test_id(6);
    let mut u64_0: u64 = 6278u64;
    let mut rng_0: crate::Rng = crate::Rng::default();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_1: u64 = 3750u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u64_2: u64 = 4161u64;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::new();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::default();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut u64_3: u64 = 7180u64;
    let mut rng_7: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut u64_4: u64 = 1255u64;
    crate::global_rng::seed(u64_4);
    let mut f64_0: f64 = crate::Rng::f64(rng_7_ref_0);
    let mut option_0: std::option::Option<u64> = crate::global_rng::random_seed();
    let mut u64_5: u64 = std::option::Option::unwrap(option_0);
    let mut char_0: char = crate::Rng::alphabetic(rng_3_ref_0);
    let mut char_1: char = crate::Rng::alphanumeric(rng_2_ref_0);
    crate::global_rng::seed(u64_2);
    let mut bool_0: bool = crate::Rng::bool(rng_1_ref_0);
    let mut char_2: char = crate::global_rng::lowercase();
    crate::Rng::seed(rng_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_9() {
    rusty_monitor::set_test_id(9);
    let mut u32_0: u32 = 2268u32;
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u32_1: u32 = 7565u32;
    let mut rng_1: crate::Rng = crate::Rng::default();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u64_0: u64 = 5687u64;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u32_2: u32 = 4105u32;
    let mut rng_3: crate::Rng = crate::Rng::new();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u64_1: u64 = 1514u64;
    let mut u64_2: u64 = 7767u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_4_ref_0: &crate::Rng = &mut rng_4;
    let mut u64_3: u64 = 4064u64;
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_5_ref_0: &crate::Rng = &mut rng_5;
    let mut option_0: std::option::Option<u64> = crate::global_rng::random_seed();
    let mut u64_4: u64 = crate::global_rng::get_seed();
    let mut u64_5: u64 = crate::global_rng::get_seed();
    let mut bool_0: bool = crate::Rng::eq(rng_5_ref_0, rng_4_ref_0);
    let mut char_0: char = crate::global_rng::uppercase();
    let mut u64_6: u64 = std::option::Option::unwrap(option_0);
    let mut rng_6: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut char_1: char = crate::Rng::lowercase(rng_3_ref_0);
    let mut char_2: char = crate::global_rng::digit(u32_2);
    let mut f32_0: f32 = crate::global_rng::f32();
    let mut char_3: char = crate::global_rng::alphabetic();
    crate::Rng::seed(rng_2_ref_0, u64_0);
    let mut char_4: char = crate::Rng::digit(rng_1_ref_0, u32_1);
    let mut char_5: char = crate::Rng::digit(rng_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_19() {
    rusty_monitor::set_test_id(19);
    let mut u32_0: u32 = 8945u32;
    let mut rng_0: crate::Rng = crate::Rng::default();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_0: u64 = 2228u64;
    let mut u64_1: u64 = 5759u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut u64_2: u64 = 2979u64;
    let mut u128_0: u128 = 7680u128;
    let mut rng_3: crate::Rng = crate::Rng::new();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u64_3: u64 = 229u64;
    let mut u64_4: u64 = 2320u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_4);
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_6: crate::Rng = crate::Rng::new();
    let mut f64_0: f64 = crate::Rng::f64(rng_3_ref_0);
    let mut rng_7: crate::Rng = crate::Rng::default();
    let mut option_0: std::option::Option<u64> = crate::global_rng::random_seed();
    let mut rng_8: crate::Rng = crate::Rng::default();
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    crate::global_rng::seed(u64_2);
    let mut f32_0: f32 = crate::Rng::f32(rng_1_ref_0);
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    crate::Rng::seed(rng_2_ref_0, u64_0);
    let mut char_0: char = crate::Rng::lowercase(rng_0_ref_0);
    let mut rng_8_ref_0: &mut crate::Rng = &mut rng_8;
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_23() {
    rusty_monitor::set_test_id(23);
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_3: crate::Rng = crate::Rng::new();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u32_0: u32 = 3322u32;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut u64_0: u64 = 3851u64;
    crate::global_rng::seed(u64_0);
    let mut bool_0: bool = crate::global_rng::bool();
    let mut option_0: std::option::Option<u64> = crate::global_rng::random_seed();
    let mut char_0: char = crate::global_rng::alphanumeric();
    let mut f64_0: f64 = crate::global_rng::f64();
    let mut f64_1: f64 = crate::Rng::f64(rng_5_ref_0);
    let mut f64_2: f64 = crate::global_rng::f64();
    let mut u64_1: u64 = std::option::Option::unwrap(option_0);
    let mut bool_1: bool = crate::global_rng::bool();
    let mut char_1: char = crate::global_rng::digit(u32_0);
    let mut char_2: char = crate::global_rng::alphanumeric();
    let mut bool_2: bool = crate::Rng::bool(rng_3_ref_0);
    let mut f64_3: f64 = crate::global_rng::f64();
    let mut char_3: char = crate::Rng::lowercase(rng_1_ref_0);
    let mut f64_4: f64 = crate::global_rng::f64();
    let mut u64_2: u64 = crate::Rng::get_seed(rng_0_ref_0);
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut char_4: char = crate::Rng::alphanumeric(rng_2_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_25() {
    rusty_monitor::set_test_id(25);
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u64_0: u64 = 8257u64;
    let mut rng_2: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_2_ref_0: &crate::Rng = &mut rng_2;
    let mut u64_1: u64 = 5302u64;
    let mut rng_3: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u64_2: u64 = 4762u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u64_3: u64 = 2504u64;
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::new();
    let mut u64_4: u64 = 4085u64;
    let mut rng_7: crate::Rng = crate::Rng::with_seed(u64_4);
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut char_0: char = crate::Rng::alphanumeric(rng_7_ref_0);
    let mut char_1: char = crate::Rng::uppercase(rng_5_ref_0);
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut char_2: char = crate::Rng::lowercase(rng_6_ref_0);
    let mut char_3: char = crate::Rng::alphabetic(rng_4_ref_0);
    let mut f32_0: f32 = crate::Rng::f32(rng_3_ref_0);
    let mut option_0: std::option::Option<u64> = crate::global_rng::random_seed();
    let mut u64_5: u64 = std::option::Option::unwrap(option_0);
    let mut char_4: char = crate::global_rng::uppercase();
    let mut bool_0: bool = crate::Rng::bool(rng_1_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut u64_0: u64 = 376u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_1: u64 = 8219u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_1_ref_0: &crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::new();
    let mut rng_4_ref_0: &crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut u32_0: u32 = 241u32;
    let mut u64_2: u64 = 6606u64;
    let mut rng_6: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut u64_3: u64 = 4597u64;
    let mut rng_7: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut f32_0: f32 = crate::Rng::f32(rng_7_ref_0);
    let mut f64_0: f64 = crate::global_rng::f64();
    let mut char_0: char = crate::Rng::digit(rng_6_ref_0, u32_0);
    let mut f32_1: f32 = crate::global_rng::f32();
    let mut bool_0: bool = crate::Rng::bool(rng_5_ref_0);
    let mut bool_1: bool = crate::Rng::eq(rng_4_ref_0, rng_3_ref_0);
    let mut char_1: char = crate::Rng::alphabetic(rng_2_ref_0);
    let mut f64_1: f64 = crate::global_rng::f64();
    let mut rng_8: crate::Rng = crate::Rng::new();
    let mut tuple_0: () = crate::Rng::assert_receiver_is_total_eq(rng_1_ref_0);
    let mut f32_2: f32 = crate::Rng::f32(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_38() {
    rusty_monitor::set_test_id(38);
    let mut u64_0: u64 = 2974u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_0_ref_0: &crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::default();
    let mut rng_1_ref_0: &crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut u32_0: u32 = 3742u32;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u64_1: u64 = 5366u64;
    let mut u64_2: u64 = 9896u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u64_3: u64 = 6480u64;
    let mut u64_4: u64 = 6312u64;
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_4);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::default();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut u32_1: u32 = 9631u32;
    let mut rng_7: crate::Rng = crate::Rng::default();
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut char_0: char = crate::Rng::digit(rng_7_ref_0, u32_1);
    let mut f32_0: f32 = crate::global_rng::f32();
    let mut f64_0: f64 = crate::Rng::f64(rng_6_ref_0);
    crate::Rng::seed(rng_5_ref_0, u64_3);
    let mut bool_0: bool = crate::Rng::bool(rng_4_ref_0);
    crate::global_rng::seed(u64_1);
    let mut char_1: char = crate::Rng::digit(rng_3_ref_0, u32_0);
    let mut bool_1: bool = crate::Rng::eq(rng_1_ref_0, rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_41() {
    rusty_monitor::set_test_id(41);
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut u32_0: u32 = 487u32;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u64_0: u64 = 5071u64;
    let mut u64_1: u64 = 3005u64;
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut u32_1: u32 = 6297u32;
    let mut u32_2: u32 = 8149u32;
    let mut rng_6: crate::Rng = crate::Rng::default();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut u64_2: u64 = 8071u64;
    let mut rng_7: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_7_ref_0: &crate::Rng = &mut rng_7;
    let mut tuple_0: () = crate::Rng::assert_receiver_is_total_eq(rng_7_ref_0);
    let mut char_0: char = crate::Rng::digit(rng_6_ref_0, u32_2);
    let mut char_1: char = crate::global_rng::digit(u32_1);
    let mut option_0: std::option::Option<u64> = crate::global_rng::random_seed();
    crate::Rng::seed(rng_5_ref_0, u64_0);
    let mut char_2: char = crate::Rng::digit(rng_4_ref_0, u32_0);
    let mut u64_3: u64 = std::option::Option::unwrap(option_0);
    let mut f64_0: f64 = crate::global_rng::f64();
    let mut bool_0: bool = crate::Rng::eq(rng_2_ref_0, rng_1_ref_0);
    let mut char_3: char = crate::Rng::lowercase(rng_0_ref_0);
    panic!("From RustyUnit with love");
}
}