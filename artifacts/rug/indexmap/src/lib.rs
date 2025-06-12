// We *mostly* avoid unsafe code, but `Slice` allows it for DST casting.
#![deny(unsafe_code)]
#![warn(rust_2018_idioms)]
#![no_std]

//! [`IndexMap`] is a hash table where the iteration order of the key-value
//! pairs is independent of the hash values of the keys.
//!
//! [`IndexSet`] is a corresponding hash set using the same implementation and
//! with similar properties.
//!
//! ### Highlights
//!
//! [`IndexMap`] and [`IndexSet`] are drop-in compatible with the std `HashMap`
//! and `HashSet`, but they also have some features of note:
//!
//! - The ordering semantics (see their documentation for details)
//! - Sorting methods and the [`.pop()`][IndexMap::pop] methods.
//! - The [`Equivalent`] trait, which offers more flexible equality definitions
//!   between borrowed and owned versions of keys.
//! - The [`MutableKeys`][map::MutableKeys] trait, which gives opt-in mutable
//!   access to map keys, and [`MutableValues`][set::MutableValues] for sets.
//!
//! ### Feature Flags
//!
//! To reduce the amount of compiled code in the crate by default, certain
//! features are gated behind [feature flags]. These allow you to opt in to (or
//! out of) functionality. Below is a list of the features available in this
//! crate.
//!
//! * `std`: Enables features which require the Rust standard library. For more
//!   information see the section on [`no_std`].
//! * `rayon`: Enables parallel iteration and other parallel methods.
//! * `serde`: Adds implementations for [`Serialize`] and [`Deserialize`]
//!   to [`IndexMap`] and [`IndexSet`]. Alternative implementations for
//!   (de)serializing [`IndexMap`] as an ordered sequence are available in the
//!   [`map::serde_seq`] module.
//! * `arbitrary`: Adds implementations for the [`arbitrary::Arbitrary`] trait
//!   to [`IndexMap`] and [`IndexSet`].
//! * `quickcheck`: Adds implementations for the [`quickcheck::Arbitrary`] trait
//!   to [`IndexMap`] and [`IndexSet`].
//! * `borsh` (**deprecated**): Adds implementations for [`BorshSerialize`] and
//!   [`BorshDeserialize`] to [`IndexMap`] and [`IndexSet`]. Due to a cyclic
//!   dependency that arose between [`borsh`] and `indexmap`, `borsh v1.5.6`
//!   added an `indexmap` feature that should be used instead of enabling the
//!   feature here.
//!
//! _Note: only the `std` feature is enabled by default._
//!
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
//! [`no_std`]: #no-standard-library-targets
//! [`Serialize`]: `::serde::Serialize`
//! [`Deserialize`]: `::serde::Deserialize`
//! [`BorshSerialize`]: `::borsh::BorshSerialize`
//! [`BorshDeserialize`]: `::borsh::BorshDeserialize`
//! [`borsh`]: `::borsh`
//! [`arbitrary::Arbitrary`]: `::arbitrary::Arbitrary`
//! [`quickcheck::Arbitrary`]: `::quickcheck::Arbitrary`
//!
//! ### Alternate Hashers
//!
//! [`IndexMap`] and [`IndexSet`] have a default hasher type
//! [`S = RandomState`][std::collections::hash_map::RandomState],
//! just like the standard `HashMap` and `HashSet`, which is resistant to
//! HashDoS attacks but not the most performant. Type aliases can make it easier
//! to use alternate hashers:
//!
//! ```
//! use fnv::FnvBuildHasher;
//! use indexmap::{IndexMap, IndexSet};
//!
//! type FnvIndexMap<K, V> = IndexMap<K, V, FnvBuildHasher>;
//! type FnvIndexSet<T> = IndexSet<T, FnvBuildHasher>;
//!
//! let std: IndexSet<i32> = (0..100).collect();
//! let fnv: FnvIndexSet<i32> = (0..100).collect();
//! assert_eq!(std, fnv);
//! ```
//!
//! ### Rust Version
//!
//! This version of indexmap requires Rust 1.63 or later.
//!
//! The indexmap 2.x release series will use a carefully considered version
//! upgrade policy, where in a later 2.x version, we will raise the minimum
//! required Rust version.
//!
//! ## No Standard Library Targets
//!
//! This crate supports being built without `std`, requiring `alloc` instead.
//! This is chosen by disabling the default "std" cargo feature, by adding
//! `default-features = false` to your dependency specification.
//!
//! - Creating maps and sets using [`new`][IndexMap::new] and
//!   [`with_capacity`][IndexMap::with_capacity] is unavailable without `std`.
//!   Use methods [`IndexMap::default`], [`with_hasher`][IndexMap::with_hasher],
//!   [`with_capacity_and_hasher`][IndexMap::with_capacity_and_hasher] instead.
//!   A no-std compatible hasher will be needed as well, for example
//!   from the crate `twox-hash`.
//! - Macros [`indexmap!`] and [`indexset!`] are unavailable without `std`. Use
//!   the macros [`indexmap_with_default!`] and [`indexset_with_default!`] instead.

#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

use alloc::vec::{self, Vec};

mod arbitrary;
#[macro_use]
mod macros;
#[cfg(feature = "borsh")]
mod borsh;
#[cfg(feature = "serde")]
mod serde;
mod util;

pub mod map;
pub mod set;

// Placed after `map` and `set` so new `rayon` methods on the types
// are documented after the "normal" methods.
#[cfg(feature = "rayon")]
mod rayon;

pub use crate::map::IndexMap;
pub use crate::set::IndexSet;
pub use equivalent::Equivalent;

// shared private items

/// Hash value newtype. Not larger than usize, since anything larger
/// isn't used for selecting position anyway.
#[derive(Clone, Copy, Debug, PartialEq)]
struct HashValue(usize);

impl HashValue {
    #[inline(always)]
    fn get(self) -> u64 {
        self.0 as u64
    }
}

#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}

impl<K, V> Clone for Bucket<K, V>
where
    K: Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        Bucket {
            hash: self.hash,
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }

    fn clone_from(&mut self, other: &Self) {
        self.hash = other.hash;
        self.key.clone_from(&other.key);
        self.value.clone_from(&other.value);
    }
}

impl<K, V> Bucket<K, V> {
    // field accessors -- used for `f` instead of closures in `.map(f)`
    fn key_ref(&self) -> &K {
        &self.key
    }
    fn value_ref(&self) -> &V {
        &self.value
    }
    fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }
    fn key(self) -> K {
        self.key
    }
    fn value(self) -> V {
        self.value
    }
    fn key_value(self) -> (K, V) {
        (self.key, self.value)
    }
    fn refs(&self) -> (&K, &V) {
        (&self.key, &self.value)
    }
    fn ref_mut(&mut self) -> (&K, &mut V) {
        (&self.key, &mut self.value)
    }
    fn muts(&mut self) -> (&mut K, &mut V) {
        (&mut self.key, &mut self.value)
    }
}

trait Entries {
    type Entry;
    fn into_entries(self) -> Vec<Self::Entry>;
    fn as_entries(&self) -> &[Self::Entry];
    fn as_entries_mut(&mut self) -> &mut [Self::Entry];
    fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Self::Entry]);
}

/// The error type for [`try_reserve`][IndexMap::try_reserve] methods.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TryReserveError {
    kind: TryReserveErrorKind,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum TryReserveErrorKind {
    // The standard library's kind is currently opaque to us, otherwise we could unify this.
    Std(alloc::collections::TryReserveError),
    CapacityOverflow,
    AllocError { layout: alloc::alloc::Layout },
}

// These are not `From` so we don't expose them in our public API.
impl TryReserveError {
    fn from_alloc(error: alloc::collections::TryReserveError) -> Self {
        Self {
            kind: TryReserveErrorKind::Std(error),
        }
    }

    fn from_hashbrown(error: hashbrown::TryReserveError) -> Self {
        Self {
            kind: match error {
                hashbrown::TryReserveError::CapacityOverflow => {
                    TryReserveErrorKind::CapacityOverflow
                }
                hashbrown::TryReserveError::AllocError { layout } => {
                    TryReserveErrorKind::AllocError { layout }
                }
            },
        }
    }
}

impl core::fmt::Display for TryReserveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let reason = match &self.kind {
            TryReserveErrorKind::Std(e) => return core::fmt::Display::fmt(e, f),
            TryReserveErrorKind::CapacityOverflow => {
                " because the computed capacity exceeded the collection's maximum"
            }
            TryReserveErrorKind::AllocError { .. } => {
                " because the memory allocator returned an error"
            }
        };
        f.write_str("memory allocation failed")?;
        f.write_str(reason)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for TryReserveError {}

// NOTE: This is copied from the slice module in the std lib.
/// The error type returned by [`get_disjoint_indices_mut`][`IndexMap::get_disjoint_indices_mut`].
///
/// It indicates one of two possible errors:
/// - An index is out-of-bounds.
/// - The same index appeared multiple times in the array.
//    (or different but overlapping indices when ranges are provided)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetDisjointMutError {
    /// An index provided was out-of-bounds for the slice.
    IndexOutOfBounds,
    /// Two indices provided were overlapping.
    OverlappingIndices,
}

impl core::fmt::Display for GetDisjointMutError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            GetDisjointMutError::IndexOutOfBounds => "an index is out of bounds",
            GetDisjointMutError::OverlappingIndices => "there were overlapping indices",
        };

        core::fmt::Display::fmt(msg, f)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for GetDisjointMutError {}

#[cfg(test)]
mod tests_llm_16_298 {
    use super::*;

use crate::*;

    #[test]
    fn test_key_ref() {
        let key = "test_key";
        let value = "test_value";
        let hash_value = HashValue(42);
        let bucket = Bucket {
            hash: hash_value,
            key,
            value,
        };

        assert_eq!(bucket.key_ref(), &key);
    }

    #[test]
    fn test_key_ref_with_different_data_types() {
        let key = 5;
        let value = 3.14;
        let hash_value = HashValue(24);
        let bucket = Bucket {
            hash: hash_value,
            key,
            value,
        };

        assert_eq!(bucket.key_ref(), &key);
    }
}

#[cfg(test)]
mod tests_llm_16_299 {
    use super::*;

use crate::*;

    #[test]
    fn test_key_value() {
        let key = "test_key";
        let value = "test_value";
        let hash_value = HashValue(42);
        let bucket = Bucket { hash: hash_value, key, value };

        let (returned_key, returned_value) = bucket.key_value();

        assert_eq!(returned_key, "test_key");
        assert_eq!(returned_value, "test_value");
    }
}

#[cfg(test)]
mod tests_llm_16_300 {
    use super::*;

use crate::*;

    #[test]
    fn test_muts() {
        let mut bucket = Bucket {
            hash: HashValue(42),
            key: "test_key",
            value: "test_value",
        };

        let (key_mut, value_mut) = bucket.muts();

        *key_mut = "new_key";
        *value_mut = "new_value";

        assert_eq!(bucket.key_ref(), &"new_key");
        assert_eq!(bucket.value_ref(), &"new_value");
    }
}

#[cfg(test)]
mod tests_llm_16_301 {
    use super::*;

use crate::*;

    #[test]
    fn test_ref_mut() {
        let mut bucket = Bucket {
            hash: HashValue(10),
            key: "test_key",
            value: 42,
        };

        let (key_ref, value_mut) = bucket.ref_mut();
        assert_eq!(key_ref, &"test_key");
        *value_mut += 1;
        assert_eq!(bucket.value_ref(), &43);
    }
}

#[cfg(test)]
mod tests_llm_16_302 {
    use super::*;

use crate::*;

    #[test]
    fn test_refs() {
        let key = 42;
        let value = "value";
        let bucket = Bucket {
            hash: HashValue(1),
            key,
            value,
        };

        let (ref_key, ref_value) = bucket.refs();

        assert_eq!(ref_key, &key);
        assert_eq!(ref_value, &value);
    }
}

#[cfg(test)]
mod tests_llm_16_303 {
    use super::*;

use crate::*;
    use crate::HashValue;

    #[test]
    fn test_bucket_value() {
        let key = "test_key";
        let value = "test_value";
        let hash_value = HashValue(42);
        let bucket = Bucket {
            hash: hash_value,
            key,
            value,
        };

        assert_eq!(bucket.value(), "test_value");
    }
}

#[cfg(test)]
mod tests_llm_16_304 {
    use super::*;

use crate::*;
    
    #[test]
    fn test_value_mut() {
        // Arrange
        let mut bucket = Bucket {
            hash: HashValue(1),
            key: "key",
            value: 10,
        };

        // Act
        let value_mut = bucket.value_mut();
        *value_mut = 20; // Mutate the value

        // Assert
        assert_eq!(*bucket.value_ref(), 20);
    }
}

#[cfg(test)]
mod tests_llm_16_305 {
    use super::*;

use crate::*;

    #[test]
    fn test_value_ref() {
        // Arrange
        let key = "test_key";
        let value = "test_value";
        let bucket = Bucket {
            hash: HashValue(123),
            key,
            value,
        };

        // Act
        let result = bucket.value_ref();

        // Assert
        assert_eq!(result, &"test_value");
    }
}

#[cfg(test)]
mod tests_llm_16_306 {
    use super::*;

use crate::*;

    #[test]
    fn test_hashvalue_get() {
        let hash_value = HashValue(42);
        assert_eq!(hash_value.get(), 42);
    }

    #[test]
    fn test_hashvalue_get_zero() {
        let hash_value = HashValue(0);
        assert_eq!(hash_value.get(), 0);
    }

    #[test]
    fn test_hashvalue_get_large_value() {
        let hash_value = HashValue(usize::MAX);
        assert_eq!(hash_value.get(), usize::MAX as u64);
    }
}

#[cfg(test)]
mod tests_llm_16_308 {
    use super::*;

use crate::*;
    use hashbrown::TryReserveError as HashbrownError;
    use alloc::alloc::Layout;

    #[test]
    fn test_from_hashbrown_capacity_overflow() {
        let error = HashbrownError::CapacityOverflow;
        let result = TryReserveError::from_hashbrown(error);
        match result.kind {
            TryReserveErrorKind::CapacityOverflow => (),
            _ => panic!("Expected CapacityOverflow error kind"),
        }
    }

    #[test]
    fn test_from_hashbrown_alloc_error() {
        let layout = Layout::from_size_align(1, 1).unwrap();
        let error = HashbrownError::AllocError { layout };
        let result = TryReserveError::from_hashbrown(error);
        match result.kind {
            TryReserveErrorKind::AllocError { layout: result_layout } => {
                assert_eq!(layout, result_layout);
            },
            _ => panic!("Expected AllocError error kind"),
        }
    }
}
