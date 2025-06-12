//! Functions to serialize and deserialize an [`IndexMap`] as an ordered sequence.
//!
//! The default `serde` implementation serializes `IndexMap` as a normal map,
//! but there is no guarantee that serialization formats will preserve the order
//! of the key-value pairs. This module serializes `IndexMap` as a sequence of
//! `(key, value)` elements instead, in order.
//!
//! This module may be used in a field attribute for derived implementations:
//!
//! ```
//! # use indexmap::IndexMap;
//! # use serde_derive::{Deserialize, Serialize};
//! #[derive(Deserialize, Serialize)]
//! struct Data {
//!     #[serde(with = "indexmap::map::serde_seq")]
//!     map: IndexMap<i32, u64>,
//!     // ...
//! }
//! ```

use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, Serializer};

use core::fmt::{self, Formatter};
use core::hash::{BuildHasher, Hash};
use core::marker::PhantomData;

use crate::map::Slice as MapSlice;
use crate::serde::cautious_capacity;
use crate::set::Slice as SetSlice;
use crate::IndexMap;

/// Serializes a [`map::Slice`][MapSlice] as an ordered sequence.
///
/// This behaves like [`crate::map::serde_seq`] for `IndexMap`, serializing a sequence
/// of `(key, value)` pairs, rather than as a map that might not preserve order.
impl<K, V> Serialize for MapSlice<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        serializer.collect_seq(self)
    }
}

/// Serializes a [`set::Slice`][SetSlice] as an ordered sequence.
impl<T> Serialize for SetSlice<T>
where
    T: Serialize,
{
    fn serialize<Se>(&self, serializer: Se) -> Result<Se::Ok, Se::Error>
    where
        Se: Serializer,
    {
        serializer.collect_seq(self)
    }
}

/// Serializes an [`IndexMap`] as an ordered sequence.
///
/// This function may be used in a field attribute for deriving [`Serialize`]:
///
/// ```
/// # use indexmap::IndexMap;
/// # use serde_derive::Serialize;
/// #[derive(Serialize)]
/// struct Data {
///     #[serde(serialize_with = "indexmap::map::serde_seq::serialize")]
///     map: IndexMap<i32, u64>,
///     // ...
/// }
/// ```
pub fn serialize<K, V, S, T>(map: &IndexMap<K, V, S>, serializer: T) -> Result<T::Ok, T::Error>
where
    K: Serialize,
    V: Serialize,
    T: Serializer,
{
    serializer.collect_seq(map)
}

/// Visitor to deserialize a *sequenced* `IndexMap`
struct SeqVisitor<K, V, S>(PhantomData<(K, V, S)>);

impl<'de, K, V, S> Visitor<'de> for SeqVisitor<K, V, S>
where
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
    S: Default + BuildHasher,
{
    type Value = IndexMap<K, V, S>;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "a sequenced map")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let capacity = cautious_capacity::<K, V>(seq.size_hint());
        let mut map = IndexMap::with_capacity_and_hasher(capacity, S::default());

        while let Some((key, value)) = seq.next_element()? {
            map.insert(key, value);
        }

        Ok(map)
    }
}

/// Deserializes an [`IndexMap`] from an ordered sequence.
///
/// This function may be used in a field attribute for deriving [`Deserialize`]:
///
/// ```
/// # use indexmap::IndexMap;
/// # use serde_derive::Deserialize;
/// #[derive(Deserialize)]
/// struct Data {
///     #[serde(deserialize_with = "indexmap::map::serde_seq::deserialize")]
///     map: IndexMap<i32, u64>,
///     // ...
/// }
/// ```
pub fn deserialize<'de, D, K, V, S>(deserializer: D) -> Result<IndexMap<K, V, S>, D::Error>
where
    D: Deserializer<'de>,
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
    S: Default + BuildHasher,
{
    deserializer.deserialize_seq(SeqVisitor(PhantomData))
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::iter::ExactSizeIterator;
	use std::default::Default;
	use std::iter::Iterator;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_16() {
    rusty_monitor::set_test_id(16);
    let mut usize_0: usize = 9873usize;
    let mut isize_0: isize = 28136isize;
    let mut isize_1: isize = -14477isize;
    let mut isize_2: isize = -5284isize;
    let mut isize_3: isize = -10421isize;
    let mut isize_4: isize = -4249isize;
    let mut usize_1: usize = 9084usize;
    let mut isize_5: isize = -7662isize;
    let mut isize_6: isize = -3699isize;
    let mut isize_7: isize = -9805isize;
    let mut isize_8: isize = -4086isize;
    let mut u64_0: u64 = 9446u64;
    let mut isize_9: isize = -6314isize;
    let mut isize_9_ref_0: &isize = &mut isize_9;
    let mut intovalues_0: crate::map::iter::IntoValues<isize, isize> = crate::map::iter::IntoValues::default();
    let mut intovalues_0_ref_0: &crate::map::iter::IntoValues<isize, isize> = &mut intovalues_0;
    let mut iter_0: crate::set::iter::Iter<isize> = crate::set::iter::Iter::default();
    let mut isize_10: isize = -15129isize;
    let mut isize_11: isize = -8786isize;
    let mut isize_12: isize = -6172isize;
    let mut isize_13: isize = 12072isize;
    let mut isize_13_ref_0: &isize = &mut isize_13;
    let mut usize_2: usize = 4570usize;
    let mut usize_3: usize = 1787usize;
    let mut isize_14: isize = -3137isize;
    let mut isize_15: isize = 4978isize;
    let mut isize_16: isize = -11392isize;
    let mut isize_17: isize = 10260isize;
    let mut isize_18: isize = -3196isize;
    let mut option_0: std::option::Option<&isize> = crate::set::iter::Iter::last(iter_0);
    let mut usize_4: usize = crate::map::iter::IntoValues::len(intovalues_0_ref_0);
    panic!("From RustyUnit with love");
}
}