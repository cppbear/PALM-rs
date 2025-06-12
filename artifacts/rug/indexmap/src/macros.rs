/// Create an [`IndexMap`][crate::IndexMap] from a list of key-value pairs
/// and a [`BuildHasherDefault`][core::hash::BuildHasherDefault]-wrapped custom hasher.
///
/// ## Example
///
/// ```
/// use indexmap::indexmap_with_default;
/// use fnv::FnvHasher;
///
/// let map = indexmap_with_default!{
///     FnvHasher;
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
///
/// // "a" is the first key
/// assert_eq!(map.keys().next(), Some(&"a"));
/// ```
#[macro_export]
macro_rules! indexmap_with_default {
    ($H:ty; $($key:expr => $value:expr,)+) => { $crate::indexmap_with_default!($H; $($key => $value),+) };
    ($H:ty; $($key:expr => $value:expr),*) => {{
        let builder = ::core::hash::BuildHasherDefault::<$H>::default();
        const CAP: usize = <[()]>::len(&[$({ stringify!($key); }),*]);
        #[allow(unused_mut)]
        // Specify your custom `H` (must implement Default + Hasher) as the hasher:
        let mut map = $crate::IndexMap::with_capacity_and_hasher(CAP, builder);
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[macro_export]
/// Create an [`IndexMap`][crate::IndexMap] from a list of key-value pairs
///
/// ## Example
///
/// ```
/// use indexmap::indexmap;
///
/// let map = indexmap!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
///
/// // "a" is the first key
/// assert_eq!(map.keys().next(), Some(&"a"));
/// ```
macro_rules! indexmap {
    ($($key:expr => $value:expr,)+) => { $crate::indexmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            // Note: `stringify!($key)` is just here to consume the repetition,
            // but we throw away that string literal during constant evaluation.
            const CAP: usize = <[()]>::len(&[$({ stringify!($key); }),*]);
            let mut map = $crate::IndexMap::with_capacity(CAP);
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

/// Create an [`IndexSet`][crate::IndexSet] from a list of values
/// and a [`BuildHasherDefault`][core::hash::BuildHasherDefault]-wrapped custom hasher.
///
/// ## Example
///
/// ```
/// use indexmap::indexset_with_default;
/// use fnv::FnvHasher;
///
/// let set = indexset_with_default!{
///     FnvHasher;
///     "a",
///     "b",
/// };
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
///
/// // "a" is the first value
/// assert_eq!(set.iter().next(), Some(&"a"));
/// ```
#[macro_export]
macro_rules! indexset_with_default {
    ($H:ty; $($value:expr,)+) => { $crate::indexset_with_default!($H; $($value),+) };
    ($H:ty; $($value:expr),*) => {{
        let builder = ::core::hash::BuildHasherDefault::<$H>::default();
        const CAP: usize = <[()]>::len(&[$({ stringify!($value); }),*]);
        #[allow(unused_mut)]
        // Specify your custom `H` (must implement Default + Hash) as the hasher:
        let mut set = $crate::IndexSet::with_capacity_and_hasher(CAP, builder);
        $(
            set.insert($value);
        )*
        set
    }};
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[macro_export]
/// Create an [`IndexSet`][crate::IndexSet] from a list of values
///
/// ## Example
///
/// ```
/// use indexmap::indexset;
///
/// let set = indexset!{
///     "a",
///     "b",
/// };
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
///
/// // "a" is the first value
/// assert_eq!(set.iter().next(), Some(&"a"));
/// ```
macro_rules! indexset {
    ($($value:expr,)+) => { $crate::indexset!($($value),+) };
    ($($value:expr),*) => {
        {
            // Note: `stringify!($value)` is just here to consume the repetition,
            // but we throw away that string literal during constant evaluation.
            const CAP: usize = <[()]>::len(&[$({ stringify!($value); }),*]);
            let mut set = $crate::IndexSet::with_capacity(CAP);
            $(
                set.insert($value);
            )*
            set
        }
    };
}

// generate all the Iterator methods by just forwarding to the underlying
// self.iter and mapping its element.
macro_rules! iterator_methods {
    // $map_elt is the mapping function from the underlying iterator's element
    // same mapping function for both options and iterators
    ($map_elt:expr) => {
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next().map($map_elt)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }

        fn count(self) -> usize {
            self.iter.len()
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.iter.nth(n).map($map_elt)
        }

        fn last(mut self) -> Option<Self::Item> {
            self.next_back()
        }

        fn collect<C>(self) -> C
        where
            C: FromIterator<Self::Item>,
        {
            // NB: forwarding this directly to standard iterators will
            // allow it to leverage unstable traits like `TrustedLen`.
            self.iter.map($map_elt).collect()
        }
    };
}

macro_rules! double_ended_iterator_methods {
    // $map_elt is the mapping function from the underlying iterator's element
    // same mapping function for both options and iterators
    ($map_elt:expr) => {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.iter.next_back().map($map_elt)
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            self.iter.nth_back(n).map($map_elt)
        }
    };
}

// generate `ParallelIterator` methods by just forwarding to the underlying
// self.entries and mapping its elements.
#[cfg(feature = "rayon")]
macro_rules! parallel_iterator_methods {
    // $map_elt is the mapping function from the underlying iterator's element
    ($map_elt:expr) => {
        fn drive_unindexed<C>(self, consumer: C) -> C::Result
        where
            C: UnindexedConsumer<Self::Item>,
        {
            self.entries
                .into_par_iter()
                .map($map_elt)
                .drive_unindexed(consumer)
        }

        // NB: This allows indexed collection, e.g. directly into a `Vec`, but the
        // underlying iterator must really be indexed.  We should remove this if we
        // start having tombstones that must be filtered out.
        fn opt_len(&self) -> Option<usize> {
            Some(self.entries.len())
        }
    };
}

// generate `IndexedParallelIterator` methods by just forwarding to the underlying
// self.entries and mapping its elements.
#[cfg(feature = "rayon")]
macro_rules! indexed_parallel_iterator_methods {
    // $map_elt is the mapping function from the underlying iterator's element
    ($map_elt:expr) => {
        fn drive<C>(self, consumer: C) -> C::Result
        where
            C: Consumer<Self::Item>,
        {
            self.entries.into_par_iter().map($map_elt).drive(consumer)
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn with_producer<CB>(self, callback: CB) -> CB::Output
        where
            CB: ProducerCallback<Self::Item>,
        {
            self.entries
                .into_par_iter()
                .map($map_elt)
                .with_producer(callback)
        }
    };
}

#[cfg(test)]
mod tests_llm_16_63 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_collect_from_drain() {
        let mut map = IndexMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");

        let drain = map.drain(..);
        let collected: IndexMap<i32, &str> = drain.collect();

        let mut expected = IndexMap::new();
        expected.insert(1, "one");
        expected.insert(2, "two");
        expected.insert(3, "three");

        assert_eq!(collected, expected);
        assert!(map.is_empty());
    }

    #[test]
    fn test_collect_empty_drain() {
        let mut map: IndexMap<i32, &str> = IndexMap::new();
        let drain = map.drain(..);
        let collected: IndexMap<i32, &str> = drain.collect();

        assert!(collected.is_empty());
        assert!(map.is_empty());
    }

    #[test]
    fn test_collect_partial_drain() {
        let mut map = IndexMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");

        let drain = map.drain(1..3);
        let collected: IndexMap<i32, &str> = drain.collect();

        let mut expected = IndexMap::new();
        expected.insert(2, "two");
        expected.insert(3, "three");

        assert_eq!(collected, expected);
        assert_eq!(map.len(), 1);
        assert!(map.contains_key(&1));
    }
}

#[cfg(test)]
mod tests_llm_16_72 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_nth_back() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        let mut iter = map.into_iter();

        assert_eq!(iter.nth_back(0), Some(("c", 3)));
        assert_eq!(iter.nth_back(1), Some(("b", 2)));
        assert_eq!(iter.nth_back(2), Some(("a", 1)));
        assert_eq!(iter.nth_back(3), None);
    }

    #[test]
    fn test_nth_back_on_empty() {
        let mut map: IndexMap<i32, i32> = IndexMap::new();
        let mut iter = map.into_iter();

        assert_eq!(iter.nth_back(0), None);
    }

    #[test]
    fn test_nth_back_with_multiple_calls() {
        let mut map = IndexMap::new();
        map.insert("one", 1);
        map.insert("two", 2);
        map.insert("three", 3);
        let mut iter = map.into_iter();

        assert_eq!(iter.nth_back(0), Some(("three", 3)));
        assert_eq!(iter.nth_back(0), Some(("two", 2)));
        assert_eq!(iter.nth_back(0), Some(("one", 1)));
        assert_eq!(iter.nth_back(1), None); // No more elements
    }
}

#[cfg(test)]
mod tests_llm_16_76 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_last() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let iter = map.into_iter();
        let last_entry = iter.last();

        assert_eq!(last_entry, Some(("c", 3)));
    }

    #[test]
    fn test_last_empty() {
        let map: IndexMap<i32, i32> = IndexMap::new();
        let iter = map.into_iter();
        let last_entry = iter.last();

        assert_eq!(last_entry, None);
    }
}

#[cfg(test)]
mod tests_llm_16_78 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_nth() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        
        let mut iter = map.clone().into_iter();
        
        assert_eq!(iter.nth(0), Some(("a", 1)));
        assert_eq!(iter.nth(1), Some(("b", 2)));
        assert_eq!(iter.nth(0), Some(("c", 3)));
        assert_eq!(iter.nth(2), None);
    }

    #[test]
    fn test_nth_empty() {
        let mut map: IndexMap<&str, i32> = IndexMap::new();
        let mut iter = map.into_iter();
        
        assert_eq!(iter.nth(0), None);
    }

    #[test]
    fn test_nth_out_of_bounds() {
        let mut map = IndexMap::new();
        map.insert("x", 10);
        
        let mut iter = map.into_iter();
        
        assert_eq!(iter.nth(1), None);
    }
}

#[cfg(test)]
mod tests_llm_16_79 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_size_hint() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        let iter = map.clone().into_iter(); // Create an iterator from the map

        let hint = iter.size_hint();
        assert_eq!(hint, (2, Some(2))); // there are 2 elements, size hint should return (2, Some(2))
    }

    #[test]
    fn test_size_hint_empty() {
        let map: IndexMap<i32, i32> = IndexMap::new();
        let iter = map.into_iter(); // Create an iterator from the empty map

        let hint = iter.size_hint();
        assert_eq!(hint, (0, Some(0))); // should return (0, Some(0)) for an empty iterator
    }
}

#[cfg(test)]
mod tests_llm_16_82 {
    use super::*; // Assuming this is in the same module as the definition

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_next_back() {
        let mut map = IndexMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");

        let mut keys = map.clone().into_keys();

        assert_eq!(keys.next_back(), Some(3));
        assert_eq!(keys.next_back(), Some(2));
        assert_eq!(keys.next_back(), Some(1));
        assert_eq!(keys.next_back(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_85 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_collect_from_keys() {
        let map: IndexMap<i32, &str> = IndexMap::from([(1, "one"), (2, "two"), (3, "three")]);
        let keys: Vec<_> = map.into_keys().collect();
        assert_eq!(keys, vec![1, 2, 3]);
    }

    #[test]
    fn test_collect_empty() {
        let map: IndexMap<i32, &str> = IndexMap::new();
        let keys: Vec<_> = map.into_keys().collect();
        assert!(keys.is_empty());
    }

    #[test]
    fn test_collect_with_duplicates() {
        let map: IndexMap<i32, &str> = IndexMap::from([(1, "one"), (2, "two"), (1, "three")]);
        let keys: Vec<_> = map.into_keys().collect();
        assert_eq!(keys, vec![1, 2]);
    }

    #[test]
    fn test_collect_preserves_order() {
        let map: IndexMap<char, i32> = IndexMap::from([( 'a', 1), ('b', 2), ('c', 3)]);
        let keys: Vec<_> = map.into_keys().collect();
        assert_eq!(keys, vec!['a', 'b', 'c']);
    }
}

#[cfg(test)]
mod tests_llm_16_89 {
    use super::*; // Make sure to bring the necessary items into scope

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_nth() {
        let mut map: IndexMap<i32, i32> = IndexMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        
        let mut keys_iter = map.into_keys();
        
        // Test accessing 0th index
        assert_eq!(keys_iter.nth(0), Some(1));
        
        // Test accessing 1st index
        assert_eq!(keys_iter.nth(0), Some(2));
        
        // Test accessing 2nd index
        assert_eq!(keys_iter.nth(0), Some(3));
        
        // Test accessing out of bounds
        assert_eq!(keys_iter.nth(0), None);
    }
}

#[cfg(test)]
mod tests_llm_16_94 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_nth_back() {
        let mut map = IndexMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");

        let mut values = map.into_values();
        
        assert_eq!(values.nth_back(0), Some("c"));
        assert_eq!(values.nth_back(1), Some("b"));
        assert_eq!(values.nth_back(2), Some("a"));
        assert_eq!(values.nth_back(3), None);
    }
}

#[cfg(test)]
mod tests_llm_16_97 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_count() {
        let mut map = IndexMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");

        let values = map.clone().into_values();
        assert_eq!(values.count(), 3);

        let empty_map: IndexMap<i32, &str> = IndexMap::new();
        let empty_values = empty_map.into_values();
        assert_eq!(empty_values.count(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_101 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_size_hint_empty() {
        let map: IndexMap<u32, u32> = IndexMap::new();
        let into_values = map.into_values();
        assert_eq!(into_values.size_hint(), (0, None));
    }

    #[test]
    fn test_size_hint_non_empty() {
        let mut map: IndexMap<u32, u32> = IndexMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        let into_values = map.into_values();
        assert_eq!(into_values.size_hint(), (2, Some(2)));
    }
}

#[cfg(test)]
mod tests_llm_16_105 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_next_back() {
        let mut map = IndexMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");
        map.insert("key3", "value3");

        let mut iter = map.iter();
        
        // Call next_back and check the value
        assert_eq!(iter.next_back(), Some((&"key3", &"value3")));
        assert_eq!(iter.next_back(), Some((&"key2", &"value2")));
        assert_eq!(iter.next_back(), Some((&"key1", &"value1")));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_next_back_on_empty() {
        let mut map: IndexMap<&str, &str> = IndexMap::new();
        let mut iter = map.iter();

        // Calling next_back on an empty iterator
        assert_eq!(iter.next_back(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_106 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_nth_back() {
        let mut map: IndexMap<usize, &str> = IndexMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");

        let mut iter = map.iter();
        assert_eq!(iter.nth_back(0), Some((&3, &"three")));
        assert_eq!(iter.nth_back(1), Some((&2, &"two")));
        assert_eq!(iter.nth_back(2), Some((&1, &"one")));
        assert_eq!(iter.nth_back(3), None);
    }

    #[test]
    fn test_nth_back_empty() {
        let mut map: IndexMap<usize, &str> = IndexMap::new();
        let mut iter = map.iter();
        assert_eq!(iter.nth_back(0), None);
    }

    #[test]
    fn test_nth_back_single_element() {
        let mut map: IndexMap<usize, &str> = IndexMap::new();
        map.insert(1, "one");

        let mut iter = map.iter();
        assert_eq!(iter.nth_back(0), Some((&1, &"one")));
        assert_eq!(iter.nth_back(1), None);
    }
}

#[cfg(test)]
mod tests_llm_16_111 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_next() {
        let mut map = IndexMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");

        let mut iter = map.iter();

        assert_eq!(iter.next(), Some((&"key1", &"value1")));
        assert_eq!(iter.next(), Some((&"key2", &"value2")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_next_empty() {
        let map: IndexMap<&str, &str> = IndexMap::new();
        let mut iter = map.iter();

        assert_eq!(iter.next(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_112 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_nth() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        
        let mut iter = map.iter();
        
        assert_eq!(iter.nth(0), Some((&"a", &1)));
        assert_eq!(iter.nth(1), Some((&"b", &2)));
        assert_eq!(iter.nth(1), Some((&"c", &3)));
        assert_eq!(iter.nth(1), None);
    }

    #[test]
    fn test_nth_out_of_bounds() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        let mut iter = map.iter();
        
        assert_eq!(iter.nth(10), None);
    }

    #[test]
    fn test_nth_multiple_calls() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        let mut iter = map.iter();
        
        assert_eq!(iter.nth(0), Some((&"a", &1)));
        assert_eq!(iter.nth(0), Some((&"b", &2)));
        assert_eq!(iter.nth(0), None);
    }
}

#[cfg(test)]
mod tests_llm_16_113 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_size_hint_empty() {
        let map: IndexMap<i32, i32> = IndexMap::new();
        let iter = map.iter();
        let hint = iter.size_hint();
        assert_eq!(hint, (0, Some(0))); // Empty map should return (0, Some(0))
    }

    #[test]
    fn test_size_hint_non_empty() {
        let mut map = IndexMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        let iter = map.iter();
        let hint = iter.size_hint();
        assert_eq!(hint, (2, Some(2))); // Map with two entries should return (2, Some(2))
    }

    #[test]
    fn test_size_hint_after_iteration() {
        let mut map = IndexMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        let mut iter = map.iter();
        let hint_before = iter.size_hint();
        iter.next(); // Consume one item
        let hint_after = iter.size_hint();
        assert_eq!(hint_before, (2, Some(2))); // Before consuming
        assert_eq!(hint_after, (1, Some(1))); // After consuming one item
    }
}

#[cfg(test)]
mod tests_llm_16_119 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn collect_from_empty_iterator() {
        let iter: Vec<(i32, &str)> = vec![];
        let map: IndexMap<i32, &str> = iter.into_iter().collect();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn collect_from_iterator() {
        let iter = vec![(1, "one"), (2, "two"), (3, "three")];
        let map: IndexMap<i32, &str> = iter.into_iter().collect();
        assert_eq!(map.len(), 3);
        assert_eq!(map[&1], "one");
        assert_eq!(map[&2], "two");
        assert_eq!(map[&3], "three");
    }

    #[test]
    fn collect_with_duplicate_keys() {
        let iter = vec![(1, "one"), (2, "two"), (1, "uno")];
        let map: IndexMap<i32, &str> = iter.into_iter().collect();
        assert_eq!(map.len(), 2);
        assert_eq!(map[&1], "uno");  // The last value should prevail
        assert_eq!(map[&2], "two");
    }
}

#[cfg(test)]
mod tests_llm_16_120 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_count() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let mut iter = map.iter_mut();
        let count = iter.count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_empty() {
        let mut map: IndexMap<&str, i32> = IndexMap::new();
        let mut iter = map.iter_mut();
        let count = iter.count();
        assert_eq!(count, 0);
    }
}

#[cfg(test)]
mod tests_llm_16_127 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_next_back() {
        let mut map: IndexMap<i32, i32> = IndexMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);

        let mut iter = map.iter_mut();
        
        // Call next_back and check the last element
        assert_eq!(iter.next_back(), Some((&3, &mut 30)));
        // Call next_back and check the second last element
        assert_eq!(iter.next_back(), Some((&2, &mut 20)));
        // Call next_back and check the first element
        assert_eq!(iter.next_back(), Some((&1, &mut 10)));
        // Call next_back and check that there are no more elements
        assert_eq!(iter.next_back(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_128 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_nth_back() {
        // Create an IndexMap
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        
        // Create an IterMut of the map
        let mut iter = map.iter_mut();
        
        // Test nth_back with different values of n
        assert_eq!(iter.nth_back(0), Some((&"c", &mut 3)));
        assert_eq!(iter.nth_back(1), Some((&"b", &mut 2)));
        assert_eq!(iter.nth_back(2), Some((&"a", &mut 1)));
        assert_eq!(iter.nth_back(3), None); // Out of bounds
    }
}

#[cfg(test)]
mod tests_llm_16_130 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_collect_from_iterator() {
        let mut map = IndexMap::new();
        let items = vec![(1, "one"), (2, "two"), (3, "three")];
        
        let collected: IndexMap<_, _> = items.iter().map(|&(k, v)| (k, v)).collect();
        
        map.extend(collected);
        
        assert_eq!(map.len(), 3);
        assert_eq!(map[&1], "one");
        assert_eq!(map[&2], "two");
        assert_eq!(map[&3], "three");
    }

    #[test]
    fn test_collect_empty() {
        let items: Vec<(i32, &str)> = Vec::new();
        let collected: IndexMap<_, _> = items.into_iter().collect();
        
        assert!(collected.is_empty());
    }

    #[test]
    fn test_collect_duplicates() {
        let items = vec![(1, "one"), (1, "uno"), (2, "two")];
        let collected: IndexMap<_, _> = items.into_iter().collect();
        
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[&1], "uno"); // last one should prevail
    }
}

#[cfg(test)]
mod tests_llm_16_133 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_next() {
        let mut map = IndexMap::new();
        map.insert("key1", 1);
        map.insert("key2", 2);

        let mut iter = map.iter_mut();

        // Check the first item
        if let Some((key, value)) = iter.next() {
            assert_eq!(*key, "key1");
            *value += 1; // Mutate the value
        } else {
            panic!("Expected a value, but got None");
        }

        // Check the second item
        if let Some((key, value)) = iter.next() {
            assert_eq!(*key, "key2");
            *value += 2; // Mutate the value
        } else {
            panic!("Expected a value, but got None");
        }

        // Check that the iterator is exhausted
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_next_empty() {
        let mut map: IndexMap<&str, i32> = IndexMap::new();
        let mut iter = map.iter_mut();

        // Check that the iterator is None for an empty map
        assert!(iter.next().is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_134 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_iter_mut_nth() {
        let mut map = IndexMap::new();
        map.insert("key1", 1);
        map.insert("key2", 2);
        map.insert("key3", 3);

        let mut iter = map.iter_mut();
        
        let first = iter.nth(0);
        let second = iter.nth(0);
        let third = iter.nth(0);
        let none = iter.nth(0);

        assert_eq!(first, Some((&"key1", &mut 1)));
        assert_eq!(second, Some((&"key2", &mut 2)));
        assert_eq!(third, Some((&"key3", &mut 3)));
        assert_eq!(none, None);
    }
}

#[cfg(test)]
mod tests_llm_16_135 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_size_hint() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        let mut iter = map.iter_mut();
        
        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, 2);
        assert_eq!(upper, Some(2));
        
        iter.next(); // consume one element
        
        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, 1);
        assert_eq!(upper, Some(1));
        
        iter.next(); // consume the last element
        
        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, 0);
        assert_eq!(upper, Some(0));
    }
}

#[cfg(test)]
mod tests_llm_16_139 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_next_back() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let mut keys = map.keys();
        assert_eq!(keys.next_back(), Some(&"c"));
        assert_eq!(keys.next_back(), Some(&"b"));
        assert_eq!(keys.next_back(), Some(&"a"));
        assert_eq!(keys.next_back(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_140 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_nth_back() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        
        let keys = map.keys();

        assert_eq!(keys.clone().nth_back(0), Some(&"c"));
        assert_eq!(keys.clone().nth_back(1), Some(&"b"));
        assert_eq!(keys.clone().nth_back(2), Some(&"a"));
        assert_eq!(keys.clone().nth_back(3), None);
    }
}

#[cfg(test)]
mod tests_llm_16_143 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_collect_keys() {
        let mut map = IndexMap::new();
        map.insert(1, "value1");
        map.insert(2, "value2");
        map.insert(3, "value3");

        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys, vec![&1, &2, &3]);
    }

    #[test]
    fn test_collect_entries() {
        let mut map = IndexMap::new();
        map.insert(1, "value1");
        map.insert(2, "value2");

        let entries: Vec<_> = map.iter().collect();
        assert_eq!(entries, vec![(&1, &"value1"), (&2, &"value2")]);
    }

    #[test]
    fn test_collect_empty() {
        let map: IndexMap<i32, &str> = IndexMap::new();
        let entries: Vec<_> = map.iter().collect();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_collect_after_insert() {
        let mut map = IndexMap::new();
        map.insert(4, "value4");
        map.insert(5, "value5");

        let entries: Vec<_> = map.iter().collect();
        assert_eq!(entries, vec![(&4, &"value4"), (&5, &"value5")]);
    }

    #[test]
    fn test_collect_with_updates() {
        let mut map = IndexMap::new();
        map.insert(1, "value1");
        map.insert(2, "value2");
        map.insert(1, "new_value1");

        let entries: Vec<_> = map.iter().collect();
        assert_eq!(entries, vec![(&1, &"new_value1"), (&2, &"value2")]);
    }

    #[test]
    fn test_collect_with_removal() {
        let mut map = IndexMap::new();
        map.insert(1, "value1");
        map.insert(2, "value2");
        map.remove(&1);

        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys, vec![&2]);
    }
}

#[cfg(test)]
mod tests_llm_16_144 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn count_empty_keys() {
        let map: IndexMap<i32, i32> = IndexMap::new();
        let keys = map.keys();
        assert_eq!(keys.count(), 0);
    }

    #[test]
    fn count_non_empty_keys() {
        let mut map = IndexMap::new();
        map.insert(1, 100);
        map.insert(2, 200);
        let keys = map.keys();
        assert_eq!(keys.count(), 2);
    }

    #[test]
    fn count_keys_with_duplicate_insertions() {
        let mut map = IndexMap::new();
        map.insert(1, 100);
        map.insert(1, 200); // updates the value, does not create a new key
        let keys = map.keys();
        assert_eq!(keys.count(), 1);
    }

    #[test]
    fn count_keys_after_removal() {
        let mut map = IndexMap::new();
        map.insert(1, 100);
        map.insert(2, 200);
        map.remove(&1);
        let keys = map.keys();
        assert_eq!(keys.count(), 1);
    }
}

#[cfg(test)]
mod tests_llm_16_146 {
    use super::*; // Ensure to import the necessary items from the scope

use crate::*;
    use crate::IndexMap; // Import IndexMap from the indexmap crate

    #[test]
    fn test_keys_next() {
        let mut map = IndexMap::new();
        map.insert("key1", 1);
        map.insert("key2", 2);
        map.insert("key3", 3);

        let keys = map.keys();
        let mut keys_iter = keys.clone(); // Clone to avoid consuming the iterator

        assert_eq!(keys_iter.next(), Some(&"key1"));
        assert_eq!(keys_iter.next(), Some(&"key2"));
        assert_eq!(keys_iter.next(), Some(&"key3"));
        assert_eq!(keys_iter.next(), None); // Should return None after iterating through all keys
    }

    #[test]
    fn test_keys_next_empty() {
        let map: IndexMap<&str, i32> = IndexMap::new();
        let keys_iter = map.keys();

        let mut empty_keys_iter = keys_iter.clone(); // Clone to avoid consuming the iterator

        assert_eq!(empty_keys_iter.next(), None); // Should return None since the map is empty
    }
}

#[cfg(test)]
mod tests_llm_16_148 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_size_hint_empty() {
        let map: IndexMap<i32, i32> = IndexMap::new();
        let keys = map.keys();
        assert_eq!(keys.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_size_hint_non_empty() {
        let mut map = IndexMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        let keys = map.keys();
        assert_eq!(keys.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_size_hint_after_removal() {
        let mut map = IndexMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.remove(&1);
        let keys = map.keys();
        assert_eq!(keys.size_hint(), (1, Some(1)));
    }
}

#[cfg(test)]
mod tests_llm_16_158 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_next_back() {
        let mut map: IndexMap<&str, i32> = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let mut values = map.values();

        assert_eq!(values.next_back(), Some(&3));
        assert_eq!(values.next_back(), Some(&2));
        assert_eq!(values.next_back(), Some(&1));
        assert_eq!(values.next_back(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_159 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_nth_back() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        
        let values = map.values();

        assert_eq!(values.clone().nth_back(0), Some(&3));
        assert_eq!(values.clone().nth_back(1), Some(&2));
        assert_eq!(values.clone().nth_back(2), Some(&1));
        assert_eq!(values.clone().nth_back(3), None);
    }
}

#[cfg(test)]
mod tests_llm_16_162 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_count() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        
        let values_iter = map.values();
        assert_eq!(values_iter.count(), 3);
    }

    #[test]
    fn test_count_empty() {
        let map: IndexMap<&str, i32> = IndexMap::new();
        let values_iter = map.values();
        assert_eq!(values_iter.count(), 0);
    }
    
    #[test]
    fn test_count_after_cloning() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        
        let values_iter = map.values();
        let cloned_iter = values_iter.clone();
        assert_eq!(values_iter.count(), cloned_iter.count());
    }
}

#[cfg(test)]
mod tests_llm_16_163 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_last() {
        let mut map = IndexMap::new();
        map.insert("one", 1);
        map.insert("two", 2);
        map.insert("three", 3);

        // Create a Values iterator
        let values_iter = map.values();
        
        // Test the `last` method
        assert_eq!(values_iter.clone().last(), Some(&3));
        assert_eq!(values_iter.last(), Some(&3));
    }

    #[test]
    fn test_last_empty() {
        let map: IndexMap<i32, i32> = IndexMap::new();
        
        // Create a Values iterator
        let values_iter = map.values();

        // Test the `last` method on an empty iterator
        assert_eq!(values_iter.last(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_165 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_nth_valid() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        
        let values = map.values();
        assert_eq!(values.clone().nth(0), Some(&1));
        assert_eq!(values.clone().nth(1), Some(&2));
        assert_eq!(values.clone().nth(2), Some(&3));
    }

    #[test]
    fn test_nth_out_of_bounds() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        
        let values = map.values();
        assert_eq!(values.clone().nth(3), None);
    }

    #[test]
    fn test_nth_on_empty() {
        let map: IndexMap<&str, i32> = IndexMap::new();
        let values = map.values();
        assert_eq!(values.clone().nth(0), None);
    }
}

#[cfg(test)]
mod tests_llm_16_169 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_next_back() {
        let mut map = IndexMap::new();
        map.insert("key1", 1);
        map.insert("key2", 2);
        map.insert("key3", 3);

        let mut values = map.values_mut();

        assert_eq!(values.next_back(), Some(&mut 3));
        assert_eq!(values.next_back(), Some(&mut 2));
        assert_eq!(values.next_back(), Some(&mut 1));
        assert_eq!(values.next_back(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_170 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_nth_back() {
        let mut map = IndexMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");

        let mut values = map.values_mut();
        assert_eq!(values.nth_back(0), Some(&mut "three"));
        assert_eq!(values.nth_back(1), Some(&mut "two"));
        assert_eq!(values.nth_back(2), Some(&mut "one"));
        assert_eq!(values.nth_back(3), None);
    }
}

#[cfg(test)]
mod tests_llm_16_172 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_collect_into_index_map() {
        let mut index_map = IndexMap::new();
        let vec = vec![('a', 1), ('b', 2), ('c', 3)];

        // Collecting values into IndexMap from an iterator
        index_map.extend(vec.iter().map(|&(k, v)| (k, v)));

        assert_eq!(index_map.len(), 3);
        assert_eq!(index_map[&'a'], 1);
        assert_eq!(index_map[&'b'], 2);
        assert_eq!(index_map[&'c'], 3);
    }

    #[test]
    fn test_collect_empty() {
        let index_map: IndexMap<char, i32> = IndexMap::new();
        let results: IndexMap<char, i32> = vec![]
            .into_iter()
            .collect();

        assert!(results.is_empty());
    }

    #[test]
    fn test_collect_duplicates() {
        let mut index_map = IndexMap::new();
        let vec = vec![('a', 1), ('a', 2), ('b', 3)];

        index_map.extend(vec.iter().map(|&(k, v)| (k, v)));

        // Last value for 'a' should prevail
        assert_eq!(index_map[&'a'], 2);
        assert_eq!(index_map[&'b'], 3);
        assert_eq!(index_map.len(), 2);
    }
}

#[cfg(test)]
mod tests_llm_16_173 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn count_test() {
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        
        let mut values_mut = map.values_mut();
        assert_eq!(values_mut.count(), 3);
        
        // Remove one entry
        map.remove("b");

        // Re-fetch values_mut after the removal
        let mut values_mut = map.values_mut();
        assert_eq!(values_mut.count(), 2);
    }
}

#[cfg(test)]
mod tests_llm_16_175 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_next() {
        let mut map: IndexMap<i32, i32> = IndexMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        
        let mut values_iter = map.values_mut();
        
        // Test that next returns Some mutable references
        assert_eq!(*values_iter.next().unwrap(), 10);
        assert_eq!(*values_iter.next().unwrap(), 20);
        assert_eq!(*values_iter.next().unwrap(), 30);
        
        // Test that next returns None after all values have been iterated
        assert!(values_iter.next().is_none());
    }
    
    #[test]
    fn test_next_empty() {
        let mut map: IndexMap<i32, i32> = IndexMap::new();
        let mut values_iter = map.values_mut();
        
        // Test that next returns None for an empty map
        assert!(values_iter.next().is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_177 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_size_hint_empty() {
        let mut map: IndexMap<i32, i32> = IndexMap::new();
        let values_mut = map.values_mut();
        assert_eq!(values_mut.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_size_hint_non_empty() {
        let mut map = IndexMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        let values_mut = map.values_mut();
        assert_eq!(values_mut.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_size_hint_after_mutation() {
        let mut map = IndexMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        {
            let mut values_mut = map.values_mut();
            assert_eq!(values_mut.size_hint(), (2, Some(2)));
            let value = values_mut.next().unwrap();
            *value += 5;
        }
        let values_mut = map.values_mut();
        assert_eq!(values_mut.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_size_hint_iterate_empty_after_mutation() {
        let mut map: IndexMap<i32, i32> = IndexMap::new();
        let mut values_mut = map.values_mut();
        assert_eq!(values_mut.size_hint(), (0, Some(0)));
        values_mut.next();
        assert_eq!(values_mut.size_hint(), (0, Some(0)));
    }
}

#[cfg(test)]
mod tests_llm_16_227 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_collect_with_indexmap() {
        // Create an IndexMap
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        // Collect into a Vec
        let collected: Vec<_> = map.iter().collect();
        assert_eq!(collected.len(), 3);
        assert_eq!(collected[0], (&"a", &1));
        assert_eq!(collected[1], (&"b", &2));
        assert_eq!(collected[2], (&"c", &3));
    }

    #[test]
    fn test_collect_empty() {
        // Create an empty IndexMap
        let map: IndexMap<&str, i32> = IndexMap::new();
        
        // Collect into a Vec
        let collected: Vec<_> = map.iter().collect();
        assert!(collected.is_empty());
    }

    #[test]
    fn test_collect_from_empty_drain() {
        // Create an empty IndexMap
        let mut map: IndexMap<&str, i32> = IndexMap::new();
        
        // Drain and collect into a Vec
        let drained: Vec<_> = map.drain(..).collect();
        assert!(drained.is_empty());
    }

    #[test]
    fn test_collect_with_drain() {
        // Create an IndexMap
        let mut map = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        // Drain and collect
        let drained: Vec<_> = map.drain(..).collect();
        assert_eq!(drained.len(), 3);
        assert!(map.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_240 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_next_back() {
        let mut index_set: IndexSet<i32> = IndexSet::new();
        index_set.insert(1);
        index_set.insert(2);
        index_set.insert(3);
        
        let mut iter = index_set.clone().into_iter();
        
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_next_back_empty() {
        let mut index_set: IndexSet<i32> = IndexSet::new();
        let mut iter = index_set.clone().into_iter();
        
        assert_eq!(iter.next_back(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_243 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn collect_into_indexmap() {
        let map: IndexMap<u32, u32> = vec![(1, 2), (3, 4)].into_iter().collect();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&1), Some(&2));
        assert_eq!(map.get(&3), Some(&4));
    }

    #[test]
    fn collect_empty_into_indexmap() {
        let map: IndexMap<u32, u32> = Vec::<(u32, u32)>::new().into_iter().collect();
        assert!(map.is_empty());
    }

    #[test]
    fn collect_from_empty_iterator() {
        let map: IndexMap<u32, u32> = std::iter::empty::<(u32, u32)>().collect();
        assert!(map.is_empty());
    }

    #[test]
    fn collect_from_iterator_with_duplicates() {
        let map: IndexMap<u32, u32> = vec![(1, 2), (1, 3), (2, 4)].into_iter().collect();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&1), Some(&3)); // The last value for key 1 should be stored
        assert_eq!(map.get(&2), Some(&4));
    }

    #[test]
    fn collect_with_multiple_pulls() {
        let data = vec![(1, 2), (3, 4), (5, 6)];
        let map: IndexMap<u32, u32> = data.iter().cloned().collect();
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&1), Some(&2));
        assert_eq!(map.get(&3), Some(&4));
        assert_eq!(map.get(&5), Some(&6));
    }
}

#[cfg(test)]
mod tests_llm_16_244 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_count() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let into_iter = set.clone().into_iter();
        assert_eq!(into_iter.count(), 3);

        let empty_iter: IndexSet<i32> = IndexSet::new();
        let into_iter_empty = empty_iter.into_iter();
        assert_eq!(into_iter_empty.count(), 0);

        let single_item_set: IndexSet<i32> = IndexSet::from([42]);
        let single_item_iter = single_item_set.into_iter();
        assert_eq!(single_item_iter.count(), 1);
    }
}

#[cfg(test)]
mod tests_llm_16_246 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_next() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let mut iter = set.into_iter();
        
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_next_empty() {
        let mut set: IndexSet<i32> = IndexSet::new();
        let mut iter = set.into_iter();

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_next_after_consuming() {
        let mut set = IndexSet::new();
        set.insert(10);
        set.insert(20);
        let mut iter = set.into_iter();

        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None); // Ensure we still get None after consuming
    }
}

#[cfg(test)]
mod tests_llm_16_247 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_nth() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        
        let mut iter = set.into_iter();
        
        assert_eq!(iter.nth(0), Some(1));
        assert_eq!(iter.nth(1), Some(3));
        assert_eq!(iter.nth(0), None);
    }

    #[test]
    fn test_nth_out_of_bounds() {
        let mut set: IndexSet<i32> = IndexSet::new();
        set.insert(10);
        set.insert(20);
        set.insert(30);
        
        let mut iter = set.into_iter();
        
        assert_eq!(iter.nth(5), None);
    }
}

#[cfg(test)]
mod tests_llm_16_252 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_next_back() {
        let mut set = IndexSet::new();
        set.insert("a");
        set.insert("b");
        set.insert("c");
        
        let mut iter = set.iter();

        assert_eq!(iter.next_back(), Some(&"c"));
        assert_eq!(iter.next_back(), Some(&"b"));
        assert_eq!(iter.next_back(), Some(&"a"));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_next_back_on_empty() {
        let mut set: IndexSet<&str> = IndexSet::new();
        let mut iter = set.iter();

        assert_eq!(iter.next_back(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_253 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_nth_back() {
        let mut set = IndexSet::new();
        set.insert("a");
        set.insert("b");
        set.insert("c");

        let mut iter = set.iter();
        
        assert_eq!(iter.nth_back(0), Some(&"c"));
        assert_eq!(iter.nth_back(1), Some(&"b"));
        assert_eq!(iter.nth_back(2), Some(&"a"));
        assert_eq!(iter.nth_back(3), None);
    }

    #[test]
    fn test_nth_back_empty() {
        let mut set: IndexSet<&str> = IndexSet::new();
        let mut iter = set.iter();
        
        assert_eq!(iter.nth_back(0), None);
    }

    #[test]
    fn test_nth_back_single_element() {
        let mut set = IndexSet::new();
        set.insert("a");
        
        let mut iter = set.iter();
        
        assert_eq!(iter.nth_back(0), Some(&"a"));
        assert_eq!(iter.nth_back(1), None);
    }
}

#[cfg(test)]
mod tests_llm_16_255 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_collect_from_iter() {
        let input: Vec<(i32, &str)> = vec![(1, "one"), (2, "two"), (3, "three")];
        let collected: IndexMap<i32, &str> = input.into_iter().collect();

        assert_eq!(collected.len(), 3);
        assert_eq!(collected[&1], "one");
        assert_eq!(collected[&2], "two");
        assert_eq!(collected[&3], "three");
    }

    #[test]
    fn test_collect_empty() {
        let input: Vec<(i32, &str)> = vec![];
        let collected: IndexMap<i32, &str> = input.into_iter().collect();

        assert_eq!(collected.len(), 0);
    }

    #[test]
    fn test_collect_duplicates() {
        let input: Vec<(i32, &str)> = vec![(1, "one"), (1, "uno"), (2, "two")];
        let collected: IndexMap<i32, &str> = input.into_iter().collect();

        assert_eq!(collected.len(), 2);
        assert_eq!(collected[&1], "uno"); // The last value for key `1` should prevail
        assert_eq!(collected[&2], "two");
    }
}

#[cfg(test)]
mod tests_llm_16_256 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_count() {
        let mut set = IndexSet::new();
        set.insert("a");
        set.insert("b");
        set.insert("c");

        let iter = set.iter();
        assert_eq!(iter.count(), 3);
    }

    #[test]
    fn test_count_empty() {
        let set: IndexSet<&str> = IndexSet::new();
        let iter = set.iter();
        assert_eq!(iter.count(), 0);
    }

    #[test]
    fn test_count_with_duplicates() {
        let mut set = IndexSet::new();
        set.insert("a");
        set.insert("b");
        set.insert("a"); // IndexSet does not allow duplicates

        let iter = set.iter();
        assert_eq!(iter.count(), 2); // Only "a" and "b" are unique
    }
}

#[cfg(test)]
mod tests_llm_16_257 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_last() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        
        let iter = set.iter();
        assert_eq!(iter.clone().last(), Some(&3));
    }

    #[test]
    fn test_last_empty() {
        let set: IndexSet<i32> = IndexSet::new();
        let iter = set.iter();
        assert_eq!(iter.clone().last(), None);
    }

    #[test]
    fn test_last_single_element() {
        let mut set = IndexSet::new();
        set.insert(1);
        
        let iter = set.iter();
        assert_eq!(iter.clone().last(), Some(&1));
    }
}

#[cfg(test)]
mod tests_llm_16_258 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_next() {
        let mut set = IndexSet::new();
        set.insert("a");
        set.insert("b");
        set.insert("c");

        let mut iter = set.iter();
        
        assert_eq!(iter.next(), Some(&"a"));
        assert_eq!(iter.next(), Some(&"b"));
        assert_eq!(iter.next(), Some(&"c"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_next_empty() {
        let set: IndexSet<&str> = IndexSet::new();
        let mut iter = set.iter();
        
        assert_eq!(iter.next(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_259 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_nth() {
        let mut set = IndexSet::new();
        set.insert("a");
        set.insert("b");
        set.insert("c");
        
        let mut iter = set.iter();
        
        assert_eq!(iter.nth(0), Some(&"a"));
        assert_eq!(iter.nth(0), Some(&"b"));
        assert_eq!(iter.nth(1), Some(&"c"));
        assert_eq!(iter.nth(1), None);
    }

    #[test]
    fn test_nth_out_of_bounds() {
        let mut set = IndexSet::new();
        set.insert("a");
        
        let mut iter = set.iter();
        
        assert_eq!(iter.nth(1), None);
    }

    #[test]
    fn test_nth_multiple_calls() {
        let mut set = IndexSet::new();
        set.insert("a");
        set.insert("b");
        set.insert("c");
        
        let mut iter = set.iter();
        
        assert_eq!(iter.nth(0), Some(&"a"));
        assert_eq!(iter.nth(1), Some(&"b"));
        assert_eq!(iter.nth(1), None);
    }
}

#[cfg(test)]
mod tests_llm_16_260 {
    use super::*;

use crate::*;
    use crate::IndexSet;

    #[test]
    fn test_size_hint_empty() {
        let set: IndexSet<i32> = IndexSet::new();
        let iter = set.iter();
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_size_hint_single_element() {
        let mut set = IndexSet::new();
        set.insert(1);
        let iter = set.iter();
        assert_eq!(iter.size_hint(), (1, Some(1)));
    }

    #[test]
    fn test_size_hint_multiple_elements() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        let iter = set.iter();
        assert_eq!(iter.size_hint(), (3, Some(3)));
    }

    #[test]
    fn test_size_hint_after_consuming_elements() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        let mut iter = set.iter();
        let _ = iter.next(); // consume one element
        assert_eq!(iter.size_hint(), (1, Some(1)));
    }

    #[test]
    fn test_size_hint_with_empty_after_iter() {
        let mut set = IndexSet::new();
        set.insert(1);
        set.insert(2);
        let mut iter = set.iter();
        let _ = iter.next(); // consume one element
        let _ = iter.next(); // consume the second element
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }
}
