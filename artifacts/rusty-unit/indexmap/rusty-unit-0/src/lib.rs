// We *mostly* avoid unsafe code, but `Slice` allows it for DST casting.
#![deny(unsafe_code)]
#![warn(rust_2018_idioms)]
// #![no_std]

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

pub use ntest::timeout;
pub mod rusty_monitor;

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::iter::ExactSizeIterator;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::iter::Iterator;
	use std::ops::Index;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_0() {
    rusty_monitor::set_test_id(0);
    let mut isize_0: isize = -5492isize;
    let mut isize_1: isize = -11993isize;
    let mut isize_2: isize = -7285isize;
    let mut intoiter_0: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &mut crate::map::iter::IntoIter<isize, isize> = &mut intoiter_0;
    let mut isize_3: isize = -2615isize;
    let mut isize_4: isize = -9464isize;
    let mut tryreserveerrorkind_0: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut tryreserveerror_0: crate::TryReserveError = crate::TryReserveError {kind: tryreserveerrorkind_0};
    let mut tryreserveerror_0_ref_0: &crate::TryReserveError = &mut tryreserveerror_0;
    let mut values_0: crate::map::iter::Values<isize, std::result::Result<(), crate::TryReserveError>> = crate::map::iter::Values::default();
    let mut values_0_ref_0: &crate::map::iter::Values<isize, std::result::Result<(), crate::TryReserveError>> = &mut values_0;
    let mut tryreserveerrorkind_1: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut tryreserveerror_1: crate::TryReserveError = crate::TryReserveError {kind: tryreserveerrorkind_1};
    let mut tryreserveerror_1_ref_0: &crate::TryReserveError = &mut tryreserveerror_1;
    let mut usize_0: usize = 5308usize;
    let mut usize_1: usize = 35usize;
    let mut isize_5: isize = 7721isize;
    let mut isize_6: isize = 12841isize;
    let mut isize_7: isize = -6793isize;
    let mut isize_8: isize = 1484isize;
    let mut isize_9: isize = -5021isize;
    let mut isize_10: isize = 2528isize;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut isize_11: isize = 5381isize;
    let mut usize_2: usize = 4210usize;
    let mut option_0: std::option::Option<&mut isize> = crate::map::iter::ValuesMut::last(valuesmut_0);
    let mut isize_12: &mut isize = std::option::Option::unwrap(option_0);
    let mut tryreserveerror_2: crate::TryReserveError = crate::TryReserveError::clone(tryreserveerror_1_ref_0);
    let mut tryreserveerror_2_ref_0: &crate::TryReserveError = &mut tryreserveerror_2;
    let mut bool_0: bool = crate::TryReserveError::eq(tryreserveerror_2_ref_0, tryreserveerror_0_ref_0);
    let mut option_1: std::option::Option<(isize, isize)> = crate::map::iter::IntoIter::next(intoiter_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7() {
    rusty_monitor::set_test_id(7);
    let mut intoiter_0: crate::set::iter::IntoIter<isize> = crate::set::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &crate::set::iter::IntoIter<isize> = &mut intoiter_0;
    let mut iter_0: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_0_ref_0: &crate::map::iter::Iter<isize, isize> = &mut iter_0;
    let mut getdisjointmuterror_0: GetDisjointMutError = crate::GetDisjointMutError::IndexOutOfBounds;
    let mut getdisjointmuterror_0_ref_0: &GetDisjointMutError = &mut getdisjointmuterror_0;
    let mut tryreserveerrorkind_0: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut tryreserveerror_0: crate::TryReserveError = crate::TryReserveError {kind: tryreserveerrorkind_0};
    let mut tryreserveerror_0_ref_0: &crate::TryReserveError = &mut tryreserveerror_0;
    let mut isize_0: isize = -8240isize;
    let mut intoiter_1: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut intoiter_1_ref_0: &crate::map::iter::IntoIter<isize, isize> = &mut intoiter_1;
    let mut isize_1: isize = -1376isize;
    let mut isize_2: isize = 8511isize;
    let mut iter_1: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_1_ref_0: &crate::map::iter::Iter<isize, isize> = &mut iter_1;
    let mut isize_3: isize = -5906isize;
    let mut isize_4: isize = -13506isize;
    let mut isize_5: isize = -498isize;
    let mut isize_5_ref_0: &isize = &mut isize_5;
    let mut iter_2: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_2_ref_0: &crate::set::iter::Iter<isize> = &mut iter_2;
    let mut iter_3: crate::set::iter::Iter<isize> = crate::set::iter::Iter::clone(iter_2_ref_0);
    let mut iter_3_ref_0: &crate::set::iter::Iter<isize> = &mut iter_3;
    let mut usize_0: usize = crate::set::iter::Iter::len(iter_3_ref_0);
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut usize_1: usize = crate::map::iter::Iter::len(iter_1_ref_0);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::IntoIter::size_hint(intoiter_1_ref_0);
    let mut tryreserveerror_1: crate::TryReserveError = crate::TryReserveError::clone(tryreserveerror_0_ref_0);
    let mut getdisjointmuterror_1: GetDisjointMutError = crate::GetDisjointMutError::clone(getdisjointmuterror_0_ref_0);
    let mut iter_4: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::clone(iter_0_ref_0);
    let mut slice_0: &crate::set::slice::Slice<isize> = crate::set::iter::IntoIter::as_slice(intoiter_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_11() {
    rusty_monitor::set_test_id(11);
    let mut iter_0: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_0_ref_0: &mut crate::map::iter::Iter<isize, isize> = &mut iter_0;
    let mut isize_0: isize = 393isize;
    let mut isize_1: isize = -23183isize;
    let mut tryreserveerrorkind_0: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut tryreserveerrorkind_0_ref_0: &TryReserveErrorKind = &mut tryreserveerrorkind_0;
    let mut isize_2: isize = 5871isize;
    let mut usize_0: usize = 7127usize;
    let mut itermut2_0: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut itermut2_0_ref_0: &mut crate::map::iter::IterMut2<isize, isize> = &mut itermut2_0;
    let mut iter_1: crate::set::iter::Iter<std::result::Result<(), crate::TryReserveError>> = crate::set::iter::Iter::default();
    let mut iter_1_ref_0: &crate::set::iter::Iter<std::result::Result<(), crate::TryReserveError>> = &mut iter_1;
    let mut intoiter_0: crate::set::iter::IntoIter<isize> = crate::set::iter::IntoIter::default();
    let mut isize_3: isize = 2478isize;
    let mut usize_1: usize = 4632usize;
    let mut isize_4: isize = -4943isize;
    let mut isize_5: isize = -921isize;
    let mut usize_2: usize = 5823usize;
    let mut isize_6: isize = 16881isize;
    let mut isize_6_ref_0: &isize = &mut isize_6;
    let mut intokeys_0: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut intokeys_0_ref_0: &mut crate::map::iter::IntoKeys<isize, isize> = &mut intokeys_0;
    let mut option_0: std::option::Option<isize> = crate::map::iter::IntoKeys::next(intokeys_0_ref_0);
    let mut isize_7: isize = std::option::Option::unwrap(option_0);
    let mut intovalues_0: crate::map::iter::IntoValues<isize, isize> = crate::map::iter::IntoValues::default();
    let mut tryreserveerrorkind_1: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut usize_3: usize = crate::set::iter::IntoIter::count(intoiter_0);
    let mut option_1: std::option::Option<(&mut isize, &mut isize)> = crate::map::iter::IterMut2::nth(itermut2_0_ref_0, usize_0);
    let mut tryreserveerrorkind_2: TryReserveErrorKind = crate::TryReserveErrorKind::clone(tryreserveerrorkind_0_ref_0);
    let mut intokeys_1: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut option_2: std::option::Option<(&isize, &isize)> = crate::map::iter::Iter::next(iter_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_33() {
    rusty_monitor::set_test_id(33);
    let mut tryreserveerrorkind_0: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut tryreserveerror_0: crate::TryReserveError = crate::TryReserveError {kind: tryreserveerrorkind_0};
    let mut tryreserveerror_0_ref_0: &crate::TryReserveError = &mut tryreserveerror_0;
    let mut getdisjointmuterror_0: GetDisjointMutError = crate::GetDisjointMutError::OverlappingIndices;
    let mut getdisjointmuterror_0_ref_0: &GetDisjointMutError = &mut getdisjointmuterror_0;
    let mut usize_0: usize = 2967usize;
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut keys_0_ref_0: &crate::map::iter::Keys<isize, isize> = &mut keys_0;
    let mut keys_1: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut values_0: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::default();
    let mut intoiter_0: crate::set::iter::IntoIter<isize> = crate::set::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &crate::set::iter::IntoIter<isize> = &mut intoiter_0;
    let mut isize_0: isize = -2357isize;
    let mut isize_1: isize = -1490isize;
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_0_ref_0: &mut crate::set::iter::Iter<isize> = &mut iter_0;
    let mut tryreserveerrorkind_1: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut tryreserveerrorkind_1_ref_0: &TryReserveErrorKind = &mut tryreserveerrorkind_1;
    let mut usize_1: usize = 1949usize;
    let mut isize_2: isize = -7906isize;
    let mut isize_3: isize = 1732isize;
    let mut isize_4: isize = -12432isize;
    let mut isize_5: isize = -404isize;
    let mut isize_6: isize = 3957isize;
    let mut usize_2: usize = 3301usize;
    let mut option_0: std::option::Option<&isize> = crate::set::iter::Iter::next(iter_0_ref_0);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::set::iter::IntoIter::size_hint(intoiter_0_ref_0);
    let mut usize_3: usize = crate::map::iter::Values::count(values_0);
    let mut usize_4: usize = crate::map::iter::Keys::count(keys_1);
    let mut isize_7: &isize = crate::map::iter::Keys::index(keys_0_ref_0, usize_0);
    let mut tryreserveerror_1: crate::TryReserveError = crate::TryReserveError::clone(tryreserveerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_38() {
    rusty_monitor::set_test_id(38);
    let mut isize_0: isize = 2351isize;
    let mut isize_1: isize = 5035isize;
    let mut itermut_0: crate::map::iter::IterMut<isize, isize> = crate::map::iter::IterMut::default();
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut usize_0: usize = 3640usize;
    let mut isize_2: isize = -15133isize;
    let mut isize_3: isize = 3095isize;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut intoiter_0: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &crate::map::iter::IntoIter<isize, isize> = &mut intoiter_0;
    let mut isize_4: isize = 1102isize;
    let mut isize_5: isize = 7377isize;
    let mut iter_1: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut isize_6: isize = 12577isize;
    let mut isize_7: isize = -8107isize;
    let mut tryreserveerrorkind_0: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut tryreserveerrorkind_0_ref_0: &TryReserveErrorKind = &mut tryreserveerrorkind_0;
    let mut tryreserveerrorkind_1: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut tryreserveerrorkind_1_ref_0: &TryReserveErrorKind = &mut tryreserveerrorkind_1;
    let mut values_0: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::default();
    let mut values_0_ref_0: &mut crate::map::iter::Values<isize, isize> = &mut values_0;
    let mut option_0: std::option::Option<&isize> = crate::map::iter::Values::next(values_0_ref_0);
    let mut bool_0: bool = crate::TryReserveErrorKind::eq(tryreserveerrorkind_1_ref_0, tryreserveerrorkind_0_ref_0);
    let mut usize_1: usize = crate::map::iter::Iter::count(iter_1);
    let mut isize_8: &isize = std::option::Option::unwrap(option_0);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::IntoIter::size_hint(intoiter_0_ref_0);
    let mut option_1: std::option::Option<&mut isize> = crate::map::iter::ValuesMut::last(valuesmut_0);
    let mut isize_9: &mut isize = std::option::Option::unwrap(option_1);
    let mut usize_2: usize = crate::set::iter::Iter::count(iter_0);
    let mut option_2: std::option::Option<(&isize, &mut isize)> = crate::map::iter::IterMut::last(itermut_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_39() {
    rusty_monitor::set_test_id(39);
    let mut itermut2_0: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut itermut2_0_ref_0: &crate::map::iter::IterMut2<isize, isize> = &mut itermut2_0;
    let mut tryreserveerrorkind_0: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut tryreserveerror_0: crate::TryReserveError = crate::TryReserveError {kind: tryreserveerrorkind_0};
    let mut tryreserveerror_0_ref_0: &crate::TryReserveError = &mut tryreserveerror_0;
    let mut isize_0: isize = 7546isize;
    let mut isize_1: isize = -8495isize;
    let mut usize_0: usize = 4362usize;
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut iter_0_ref_0: &mut crate::set::iter::Iter<isize> = &mut iter_0;
    let mut intoiter_0: crate::set::iter::IntoIter<isize> = crate::set::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &mut crate::set::iter::IntoIter<isize> = &mut intoiter_0;
    let mut isize_2: isize = 370isize;
    let mut isize_3: isize = 8066isize;
    let mut isize_4: isize = 7893isize;
    let mut isize_5: isize = 18742isize;
    let mut isize_6: isize = 14259isize;
    let mut isize_7: isize = -9659isize;
    let mut itermut_0: crate::map::iter::IterMut<isize, isize> = crate::map::iter::IterMut::default();
    let mut itermut_0_ref_0: &crate::map::iter::IterMut<isize, isize> = &mut itermut_0;
    let mut itermut2_1: crate::map::iter::IterMut2<isize, isize> = crate::map::iter::IterMut2::default();
    let mut itermut2_1_ref_0: &crate::map::iter::IterMut2<isize, isize> = &mut itermut2_1;
    let mut tryreserveerrorkind_1: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut slice_0: &crate::map::slice::Slice<isize, isize> = crate::map::iter::IterMut2::as_slice(itermut2_1_ref_0);
    let mut slice_1: &crate::map::slice::Slice<isize, isize> = crate::map::iter::IterMut::as_slice(itermut_0_ref_0);
    let mut tryreserveerrorkind_1_ref_0: &TryReserveErrorKind = &mut tryreserveerrorkind_1;
    let mut option_0: std::option::Option<isize> = crate::set::iter::IntoIter::next(intoiter_0_ref_0);
    let mut option_1: std::option::Option<&isize> = crate::set::iter::Iter::nth(iter_0_ref_0, usize_0);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::IterMut2::size_hint(itermut2_0_ref_0);
    let mut isize_8: &isize = std::option::Option::unwrap(option_1);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_42() {
    rusty_monitor::set_test_id(42);
    let mut iter_0: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut iter_0_ref_0: &crate::map::iter::Iter<isize, isize> = &mut iter_0;
    let mut isize_0: isize = 1796isize;
    let mut isize_1: isize = -2960isize;
    let mut isize_2: isize = -693isize;
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut usize_0: usize = 4332usize;
    let mut valuesmut_0: crate::map::iter::ValuesMut<isize, isize> = crate::map::iter::ValuesMut::default();
    let mut valuesmut_0_ref_0: &mut crate::map::iter::ValuesMut<isize, isize> = &mut valuesmut_0;
    let mut intoiter_0: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut intoiter_0_ref_0: &mut crate::map::iter::IntoIter<isize, isize> = &mut intoiter_0;
    let mut values_0: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::default();
    let mut values_0_ref_0: &crate::map::iter::Values<isize, isize> = &mut values_0;
    let mut isize_3: isize = -15895isize;
    let mut isize_4: isize = -4939isize;
    let mut tryreserveerrorkind_0: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut tryreserveerrorkind_0_ref_0: &TryReserveErrorKind = &mut tryreserveerrorkind_0;
    let mut tryreserveerrorkind_1: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut tryreserveerrorkind_1_ref_0: &TryReserveErrorKind = &mut tryreserveerrorkind_1;
    let mut bool_0: bool = crate::TryReserveErrorKind::eq(tryreserveerrorkind_1_ref_0, tryreserveerrorkind_0_ref_0);
    let mut values_1: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::clone(values_0_ref_0);
    let mut option_0: std::option::Option<(isize, isize)> = crate::map::iter::IntoIter::next(intoiter_0_ref_0);
    let mut values_1_ref_0: &crate::map::iter::Values<isize, isize> = &mut values_1;
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::Values::size_hint(values_1_ref_0);
    let mut option_1: std::option::Option<&mut isize> = crate::map::iter::ValuesMut::nth(valuesmut_0_ref_0, usize_0);
    let mut tuple_1: (isize, isize) = std::option::Option::unwrap(option_0);
    let mut usize_1: usize = crate::map::iter::Keys::count(keys_0);
    let mut iter_1: crate::map::iter::Iter<isize, isize> = crate::map::iter::Iter::default();
    let mut usize_2: usize = crate::map::iter::Iter::len(iter_0_ref_0);
    let mut iter_1_ref_0: &mut crate::map::iter::Iter<isize, isize> = &mut iter_1;
    let mut option_2: std::option::Option<(&isize, &isize)> = crate::map::iter::Iter::next(iter_1_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_43() {
    rusty_monitor::set_test_id(43);
    let mut getdisjointmuterror_0: GetDisjointMutError = crate::GetDisjointMutError::OverlappingIndices;
    let mut getdisjointmuterror_0_ref_0: &GetDisjointMutError = &mut getdisjointmuterror_0;
    let mut intoiter_0: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut isize_0: isize = -16839isize;
    let mut isize_1: isize = 10434isize;
    let mut values_0: crate::map::iter::Values<isize, isize> = crate::map::iter::Values::default();
    let mut tryreserveerrorkind_0: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut tryreserveerror_0: crate::TryReserveError = crate::TryReserveError {kind: tryreserveerrorkind_0};
    let mut tryreserveerror_0_ref_0: &crate::TryReserveError = &mut tryreserveerror_0;
    let mut getdisjointmuterror_1: GetDisjointMutError = crate::GetDisjointMutError::OverlappingIndices;
    let mut getdisjointmuterror_1_ref_0: &GetDisjointMutError = &mut getdisjointmuterror_1;
    let mut isize_2: isize = -12119isize;
    let mut usize_0: usize = 9972usize;
    let mut isize_3: isize = -661isize;
    let mut isize_4: isize = -3335isize;
    let mut isize_5: isize = -4277isize;
    let mut u64_0: u64 = 7252u64;
    let mut isize_6: isize = 6083isize;
    let mut isize_6_ref_0: &isize = &mut isize_6;
    let mut isize_7: isize = -18684isize;
    let mut isize_8: isize = 15178isize;
    let mut isize_9: isize = 18397isize;
    let mut isize_10: isize = -6189isize;
    let mut usize_1: usize = 608usize;
    let mut usize_2: usize = 2330usize;
    let mut isize_11: isize = 9612isize;
    let mut getdisjointmuterror_2: GetDisjointMutError = crate::GetDisjointMutError::clone(getdisjointmuterror_1_ref_0);
    let mut usize_3: usize = crate::map::iter::Values::count(values_0);
    let mut usize_4: usize = crate::map::iter::IntoIter::count(intoiter_0);
    let mut getdisjointmuterror_2_ref_0: &GetDisjointMutError = &mut getdisjointmuterror_2;
    let mut bool_0: bool = crate::GetDisjointMutError::eq(getdisjointmuterror_2_ref_0, getdisjointmuterror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut isize_0: isize = 14782isize;
    let mut usize_0: usize = 1674usize;
    let mut itermut_0: crate::map::iter::IterMut<isize, isize> = crate::map::iter::IterMut::default();
    let mut isize_1: isize = 9379isize;
    let mut isize_2: isize = 7720isize;
    let mut tryreserveerrorkind_0: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut tryreserveerrorkind_0_ref_0: &TryReserveErrorKind = &mut tryreserveerrorkind_0;
    let mut tryreserveerrorkind_1: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut tryreserveerrorkind_1_ref_0: &TryReserveErrorKind = &mut tryreserveerrorkind_1;
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut keys_0: crate::map::iter::Keys<isize, isize> = crate::map::iter::Keys::default();
    let mut keys_0_ref_0: &crate::map::iter::Keys<isize, isize> = &mut keys_0;
    let mut getdisjointmuterror_0: GetDisjointMutError = crate::GetDisjointMutError::IndexOutOfBounds;
    let mut getdisjointmuterror_0_ref_0: &GetDisjointMutError = &mut getdisjointmuterror_0;
    let mut intokeys_0: crate::map::iter::IntoKeys<isize, isize> = crate::map::iter::IntoKeys::default();
    let mut intokeys_0_ref_0: &crate::map::iter::IntoKeys<isize, isize> = &mut intokeys_0;
    let mut intoiter_0: crate::map::iter::IntoIter<isize, isize> = crate::map::iter::IntoIter::default();
    let mut tryreserveerrorkind_2: TryReserveErrorKind = crate::TryReserveErrorKind::CapacityOverflow;
    let mut tryreserveerrorkind_2_ref_0: &TryReserveErrorKind = &mut tryreserveerrorkind_2;
    let mut usize_1: usize = crate::map::iter::IntoIter::count(intoiter_0);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::iter::IntoKeys::size_hint(intokeys_0_ref_0);
    let mut intoiter_1: crate::set::iter::IntoIter<isize> = crate::set::iter::IntoIter::default();
    let mut getdisjointmuterror_1: GetDisjointMutError = crate::GetDisjointMutError::clone(getdisjointmuterror_0_ref_0);
    let mut getdisjointmuterror_1_ref_0: &GetDisjointMutError = &mut getdisjointmuterror_1;
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::map::iter::Keys::size_hint(keys_0_ref_0);
    let mut usize_2: usize = crate::set::iter::Iter::count(iter_0);
    let mut intoiter_1_ref_0: &crate::set::iter::IntoIter<isize> = &mut intoiter_1;
    let mut usize_3: usize = crate::set::iter::IntoIter::len(intoiter_1_ref_0);
    let mut bool_0: bool = crate::TryReserveErrorKind::eq(tryreserveerrorkind_1_ref_0, tryreserveerrorkind_0_ref_0);
    let mut slice_0: &mut crate::map::slice::Slice<isize, isize> = crate::map::iter::IterMut::into_slice(itermut_0);
    panic!("From RustyUnit with love");
}
}