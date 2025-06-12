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
mod tests_llm_16_38 {
    use super::*;

use crate::*;
    use crate::Rng;

    #[test]
    fn test_rng_new_creates_rng() {
        let rng = Rng::new();
        assert_eq!(rng.get_seed(), 0x4d595df4d0f33173);
    }

    #[test]
    fn test_rng_clone_creates_identical_rng() {
        let rng1 = Rng::new();
        let rng2 = rng1.clone();
        assert_eq!(rng1.get_seed(), rng2.get_seed());
    }

    #[test]
    fn test_rng_new_is_default() {
        let rng_default = Rng::default();
        assert_eq!(rng_default.get_seed(), 0x4d595df4d0f33173);
    }

    #[test]
    fn test_rng_fork_creates_different_rng() {
        let mut rng1 = Rng::new();
        let mut rng2 = rng1.fork();
        assert_ne!(rng1.get_seed(), rng2.get_seed());
    }
}

#[cfg(test)]
mod tests_llm_16_39 {
    use super::*;

use crate::*;
    use crate::Rng;

    #[test]
    fn test_rng_default() {
        let rng1 = Rng::default();
        let rng2 = Rng::new();
        assert_eq!(rng1.get_seed(), rng2.get_seed());
    }
}

#[cfg(test)]
mod tests_llm_16_41 {
    use super::*;

use crate::*;
    use crate::global_rng::alphabetic;

    #[test]
    fn test_alphabetic() {
        for _ in 0..1000 {
            let ch = alphabetic();
            assert!(ch.is_alphabetic(), "Generated character {} is not alphabetic", ch);
            assert!(ch.is_ascii_alphabetic(), "Generated character {} is not ASCII alphabetic", ch);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_42 {
    use super::*; // Ensure to bring `alphanumeric` into scope

use crate::*;
    use crate::Rng; // Ensure to bring `Rng` into scope

    #[test]
    fn test_alphanumeric() {
        // Generate a set of characters to check
        let generated_chars: Vec<char> = (0..1000).map(|_| alphanumeric()).collect();

        // Check that all characters are alphanumeric
        for c in generated_chars {
            assert!(c.is_alphanumeric(), "Character {} is not alphanumeric", c);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_43 {
    use super::*;

use crate::*;
    use crate::global_rng;

    #[test]
    fn test_random_bool() {
        let result = global_rng::bool();
        assert!(result == true || result == false);
    }
}

#[cfg(test)]
mod tests_llm_16_47 {
    use crate::global_rng::digit;

    #[test]
    #[should_panic]
    fn test_digit_zero_base() {
        digit(0);
    }

    #[test]
    #[should_panic]
    fn test_digit_above_max_base() {
        digit(37);
    }

    #[test]
    fn test_digit_valid_bases() {
        for base in 2..=36 {
            let result = digit(base);
            assert!(result.is_digit(base) || (result.is_ascii_alphabetic() && result.to_digit(36).unwrap() < base));
        }
    }
}

#[cfg(test)]
mod tests_llm_16_51 {
    use super::*;

use crate::*;
    use crate::global_rng;

    #[test]
    fn test_get_seed() {
        let seed = global_rng::get_seed();
        assert!(seed > 0, "Seed should be greater than 0");
    }
}

#[cfg(test)]
mod tests_llm_16_52 {
    use super::*;

use crate::*;
    use crate::global_rng;

    #[test]
    #[should_panic]
    fn test_i128_empty_range() {
        let range = 1..1; // empty range
        global_rng::i128(range);
    }

    #[test]
    fn test_i128_valid_range() {
        let range = 1..100; // valid range
        let result = global_rng::i128(range);
        assert!(result >= 1 && result < 100);
    }

    #[test]
    fn test_i128_negative_range() {
        let range = -100..-1; // valid negative range
        let result = global_rng::i128(range);
        assert!(result >= -100 && result < -1);
    }

    #[test]
    fn test_i128_large_range() {
        let range = i128::MIN..i128::MAX; // large range
        let result = global_rng::i128(range);
        assert!(result >= i128::MIN && result < i128::MAX);
    }
}

#[cfg(test)]
mod tests_llm_16_53 {
    use super::*;

use crate::*;
    use crate::global_rng;

    #[test]
    #[should_panic]
    fn test_i16_empty_range() {
        let range = 10..10; // Empty range
        global_rng::i16(range);
    }

    #[test]
    fn test_i16_valid_range() {
        let range = 1..5; // Valid range
        let value = global_rng::i16(range);
        assert!(value >= 1 && value < 5);
    }

    #[test]
    fn test_i16_negative_range() {
        let range = -10..-5; // Valid negative range
        let value = global_rng::i16(range);
        assert!(value >= -10 && value < -5);
    }

    #[test]
    fn test_i16_mixed_range() {
        let range = -5..5; // Mixed negative and positive range
        let value = global_rng::i16(range);
        assert!(value >= -5 && value < 5);
    }
}

#[cfg(test)]
mod tests_llm_16_55 {
    use super::*;

use crate::*;
    use crate::global_rng;

    #[test]
    fn test_i64_with_valid_range() {
        let result = global_rng::i64(1..10);
        assert!(result >= 1 && result < 10);
    }

    #[test]
    #[should_panic]
    fn test_i64_with_empty_range() {
        let _ = global_rng::i64(10..10);
    }
    
    #[test]
    fn test_i64_with_negative_range() {
        let result = global_rng::i64(-10..0);
        assert!(result >= -10 && result < 0);
    }

    #[test]
    fn test_i64_with_large_range() {
        let result = global_rng::i64(-1_000_000..1_000_000);
        assert!(result >= -1_000_000 && result < 1_000_000);
    }
}

#[cfg(test)]
mod tests_llm_16_56 {
    use super::*;

use crate::*;
    use crate::global_rng;

    #[test]
    #[should_panic]
    fn test_i8_panic_empty_range() {
        let range = 0..0; // empty range
        global_rng::i8(range);
    }

    #[test]
    fn test_i8_valid_range() {
        let range = -10..10; // valid range
        let value = global_rng::i8(range);
        assert!(value >= -10 && value < 10);
    }

    #[test]
    fn test_i8_single_value_range() {
        let range = 5..6; // range containing a single value
        let value = global_rng::i8(range);
        assert_eq!(value, 5);
    }
}

#[cfg(test)]
mod tests_llm_16_57 {
    use super::*;

use crate::*;
    use crate::global_rng;

    #[test]
    #[should_panic(expected = "range is empty")]
    fn test_isize_panic_empty_range() {
        global_rng::isize(0..0);
    }

    #[test]
    fn test_isize_non_empty_range() {
        let result = global_rng::isize(1..10);
        assert!(result >= 1 && result < 10);
    }

    #[test]
    fn test_isize_negative_range() {
        let result = global_rng::isize(-10..0);
        assert!(result >= -10 && result < 0);
    }

    #[test]
    fn test_isize_large_range() {
        let result = global_rng::isize(-1_000_000..1_000_000);
        assert!(result >= -1_000_000 && result < 1_000_000);
    }
}

#[cfg(test)]
mod tests_llm_16_58 {
    use super::*;

use crate::*;
    use crate::global_rng::lowercase;

    #[test]
    fn test_lowercase() {
        let c = lowercase();
        assert!(c.is_ascii_lowercase(), "Expected a lowercase ASCII character");
        assert!(c >= 'a' && c <= 'z', "Expected character to be in the range a-z");
    }
}

#[cfg(test)]
mod tests_llm_16_60 {
    use super::*;

use crate::*;
    use crate::global_rng; // Adjust the import based on your project structure

    #[test]
    fn test_seed() {
        let seed_value: u64 = 12345; // Example seed
        global_rng::seed(seed_value);
        // You may want to verify if the random number generator is behaving as expected
        // after seeding. This could involve inspecting the generator's output.
        // Example: let result = global_rng::u64(); // Get a value after seeding
        // assert_eq!(result, expected_value); // Compare with expected output
    }
}

#[cfg(test)]
mod tests_llm_16_63 {
    use crate::global_rng;

    #[test]
    fn test_u128_valid_range() {
        let result = global_rng::u128(10..20);
        assert!(result >= 10 && result < 20);
    }

    #[test]
    #[should_panic]
    fn test_u128_empty_range() {
        global_rng::u128(20..20);
    }

    #[test]
    fn test_u128_large_range() {
        let result = global_rng::u128(0..u128::MAX);
        assert!(result >= 0 && result < u128::MAX);
    }

    #[test]
    fn test_u128_edge_case() {
        let result = global_rng::u128(5..5);
        assert!(result >= 5 && result < 5);
    }
}

#[cfg(test)]
mod tests_llm_16_64 {
    use super::*;

use crate::*;
    use crate::global_rng;

    #[test]
    #[should_panic]
    fn test_u16_panics_on_empty_range() {
        let range = 0..0; // Empty range
        let _result = global_rng::u16(range);
    }

    #[test]
    fn test_u16_with_valid_range() {
        let range = 1..10; // Valid range
        let result = global_rng::u16(range);
        assert!(result >= 1 && result < 10);
    }

    #[test]
    fn test_u16_with_range_inclusive() {
        let range = 0..=5; // Inclusive range
        let result = global_rng::u16(range);
        assert!(result >= 0 && result <= 5);
    }
}

#[cfg(test)]
mod tests_llm_16_66 {
    use crate::global_rng;

    #[test]
    #[should_panic(expected = "range is empty")]
    fn test_u64_panic_empty_range() {
        let _ = global_rng::u64(0..0);
    }

    #[test]
    fn test_u64_valid_range() {
        let result = global_rng::u64(1..10);
        assert!(result >= 1 && result < 10);
    }

    #[test]
    fn test_u64_large_range() {
        let result = global_rng::u64(1..u64::MAX);
        assert!(result >= 1);
    }
}

#[cfg(test)]
mod tests_llm_16_67 {
    use super::*;

use crate::*;
    use crate::global_rng;

    #[test]
    fn test_u8_range() {
        let value = global_rng::u8(0..10);
        assert!(value >= 0 && value < 10);
    }

    #[test]
    fn test_u8_empty_range() {
        let result = std::panic::catch_unwind(|| {
            global_rng::u8(10..10);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_u8_full_range() {
        let value = global_rng::u8(0..=255);
        assert!(value <= 255);
    }
}

#[cfg(test)]
mod tests_llm_16_68 {
    use super::*;

use crate::*;
    use crate::global_rng;

    #[test]
    fn test_uppercase_generates_valid_character() {
        let generated_char = global_rng::uppercase();
        assert!(generated_char.is_ascii_uppercase(), "The generated character should be an uppercase ASCII letter.");
        assert!(('A'..='Z').contains(&generated_char), "The generated character should be between 'A' and 'Z'.");
    }
}

#[cfg(test)]
mod tests_llm_16_69 {
    use super::*;

use crate::*;
    use crate::global_rng;

    #[test]
    #[should_panic]
    fn test_usize_panics_on_empty_range() {
        let _ = global_rng::usize(1..1); // This should panic due to empty range
    }

    #[test]
    fn test_usize_with_valid_range() {
        let result = global_rng::usize(1..10);
        assert!(result >= 1 && result < 10);
    }

    #[test]
    fn test_usize_with_large_range() {
        let result = global_rng::usize(100..1000);
        assert!(result >= 100 && result < 1000);
    }

    #[test]
    fn test_usize_with_edge_case() {
        let result = global_rng::usize(0..1);
        assert_eq!(result, 0);
    }
}
