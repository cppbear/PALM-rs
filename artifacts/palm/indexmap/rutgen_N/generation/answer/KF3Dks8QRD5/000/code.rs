// Answer 0

#[test]
fn test_drain_valid_range() {
    use std::ops::{Range, RangeBounds};
    use std::vec;

    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }

        fn insert(&mut self, key: K, value: V) {
            self.entries.push(Bucket { key, value });
        }

        pub(crate) fn drain<R>(&mut self, range: R) -> vec::Drain<'_, Bucket<K, V>>
        where
            R: RangeBounds<usize>,
        {
            let start = match range.start_bound() {
                std::ops::Bound::Included(&idx) => idx,
                std::ops::Bound::Excluded(&idx) => idx + 1,
                std::ops::Bound::Unbounded => 0,
            };
            let end = match range.end_bound() {
                std::ops::Bound::Included(&idx) => idx + 1,
                std::ops::Bound::Excluded(&idx) => idx,
                std::ops::Bound::Unbounded => self.entries.len(),
            };
            self.entries.drain(start..end)
        }
    }

    let mut map = TestMap::new();
    map.insert(1, 'a');
    map.insert(2, 'b');
    map.insert(3, 'c');

    let drained: Vec<_> = map.drain(1..3).collect();
    assert_eq!(drained.len(), 2);
    assert_eq!(drained[0].key, 2);
    assert_eq!(drained[1].key, 3);
    assert_eq!(map.entries.len(), 1); // Should be left with one entry
}

#[test]
fn test_drain_empty_map() {
    use std::ops::{RangeBounds};
    use std::vec;

    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }

        pub(crate) fn drain<R>(&mut self, range: R) -> vec::Drain<'_, Bucket<K, V>>
        where
            R: RangeBounds<usize>,
        {
            let start = match range.start_bound() {
                std::ops::Bound::Included(&idx) => idx,
                std::ops::Bound::Excluded(&idx) => idx + 1,
                std::ops::Bound::Unbounded => 0,
            };
            let end = match range.end_bound() {
                std::ops::Bound::Included(&idx) => idx + 1,
                std::ops::Bound::Excluded(&idx) => idx,
                std::ops::Bound::Unbounded => self.entries.len(),
            };
            self.entries.drain(start..end)
        }
    }

    let mut map = TestMap::new();
    let drained: Vec<_> = map.drain(0..1).collect();
    assert_eq!(drained.len(), 0); // Nothing should be drained
    assert_eq!(map.entries.len(), 0); // Still empty
}

#[test]
fn test_drain_out_of_bounds() {
    use std::ops::{RangeBounds};
    use std::vec;

    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }

        fn insert(&mut self, key: K, value: V) {
            self.entries.push(Bucket { key, value });
        }

        pub(crate) fn drain<R>(&mut self, range: R) -> vec::Drain<'_, Bucket<K, V>>
        where
            R: RangeBounds<usize>,
        {
            let start = match range.start_bound() {
                std::ops::Bound::Included(&idx) => idx,
                std::ops::Bound::Excluded(&idx) => idx + 1,
                std::ops::Bound::Unbounded => 0,
            };
            let end = match range.end_bound() {
                std::ops::Bound::Included(&idx) => idx + 1,
                std::ops::Bound::Excluded(&idx) => idx,
                std::ops::Bound::Unbounded => self.entries.len(),
            };
            self.entries.drain(start..end)
        }
    }

    let mut map = TestMap::new();
    map.insert(1, 'a');
    map.insert(2, 'b');

    let drained: Vec<_> = map.drain(0..5).collect(); // Drain out of bounds
    assert_eq!(drained.len(), 2); // Should drain all entries
    assert_eq!(map.entries.len(), 0); // Should be empty after draining
}

#[test]
fn test_drain_single_element() {
    use std::ops::{RangeBounds};
    use std::vec;

    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }

        fn insert(&mut self, key: K, value: V) {
            self.entries.push(Bucket { key, value });
        }

        pub(crate) fn drain<R>(&mut self, range: R) -> vec::Drain<'_, Bucket<K, V>>
        where
            R: RangeBounds<usize>,
        {
            let start = match range.start_bound() {
                std::ops::Bound::Included(&idx) => idx,
                std::ops::Bound::Excluded(&idx) => idx + 1,
                std::ops::Bound::Unbounded => 0,
            };
            let end = match range.end_bound() {
                std::ops::Bound::Included(&idx) => idx + 1,
                std::ops::Bound::Excluded(&idx) => idx,
                std::ops::Bound::Unbounded => self.entries.len(),
            };
            self.entries.drain(start..end)
        }
    }

    let mut map = TestMap::new();
    map.insert(1, 'a');

    let drained: Vec<_> = map.drain(0..1).collect(); // Drain single element
    assert_eq!(drained.len(), 1); 
    assert_eq!(drained[0].key, 1);
    assert_eq!(map.entries.len(), 0); // Should be empty after draining
}

