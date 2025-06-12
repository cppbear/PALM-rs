// Answer 0

#[test]
fn test_split_splice_range_inside_bounds() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct TestIndexMap<K, V> {
        entries: Vec<Bucket<K, V>>,
        // Assuming we have an Indices type used for some form of indexing
        // This is a placeholder since the actual indexing structure isn't provided
        indices: Vec<usize>, 
    }

    impl<K, V> TestIndexMap<K, V> {
        fn new(entries: Vec<Bucket<K, V>>) -> Self {
            let indices = (0..entries.len()).collect();
            Self { entries, indices }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn erase_indices(&mut self, start: usize, end: usize) {
            self.entries.drain(start..end);
            self.indices.drain(start..end);
        }

        fn split_splice<R>(&mut self, range: R) -> (Self, std::vec::IntoIter<Bucket<K, V>>)
        where
            R: std::ops::RangeBounds<usize>,
        {
            // Placeholder for the actual implementation
            let (start, end) = match range.start_bound() {
                std::ops::Bound::Included(&s) => (s, match range.end_bound() {
                    std::ops::Bound::Included(&e) => e + 1,
                    std::ops::Bound::Excluded(&e) => e,
                    std::ops::Bound::Unbounded => self.len(),
                }),
                std::ops::Bound::Excluded(&s) => (s + 1, match range.end_bound() {
                    std::ops::Bound::Included(&e) => e + 1,
                    std::ops::Bound::Excluded(&e) => e,
                    std::ops::Bound::Unbounded => self.len(),
                }),
                std::ops::Bound::Unbounded => (0, self.len()),
            };

            self.erase_indices(start, end);
            let entries = self.entries.split_off(end);
            let drained = self.entries.split_off(start);

            let indices = (0..entries.len()).collect();
            (Self { indices, entries }, drained.into_iter())
        }
    }

    let mut map = TestIndexMap::new(vec![
        Bucket { key: 1, value: "a" },
        Bucket { key: 2, value: "b" },
        Bucket { key: 3, value: "c" },
        Bucket { key: 4, value: "d" },
    ]);

    let (new_map, drained) = map.split_splice(1..3);
    assert_eq!(drained.collect::<Vec<_>>().len(), 2);
    assert_eq!(new_map.len(), 2);
}

#[test]
fn test_split_splice_full_range() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct TestIndexMap<K, V> {
        entries: Vec<Bucket<K, V>>,
        // Assuming we have an Indices type used for some form of indexing
        indices: Vec<usize>
    }

    impl<K, V> TestIndexMap<K, V> {
        fn new(entries: Vec<Bucket<K, V>>) -> Self {
            let indices = (0..entries.len()).collect();
            Self { entries, indices }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn erase_indices(&mut self, start: usize, end: usize) {
            self.entries.drain(start..end);
            self.indices.drain(start..end);
        }

        fn split_splice<R>(&mut self, range: R) -> (Self, std::vec::IntoIter<Bucket<K, V>>)
        where
            R: std::ops::RangeBounds<usize>,
        {
            let (start, end) = match range.start_bound() {
                std::ops::Bound::Included(&s) => (s, match range.end_bound() {
                    std::ops::Bound::Included(&e) => e + 1,
                    std::ops::Bound::Excluded(&e) => e,
                    std::ops::Bound::Unbounded => self.len(),
                }),
                std::ops::Bound::Excluded(&s) => (s + 1, match range.end_bound() {
                    std::ops::Bound::Included(&e) => e + 1,
                    std::ops::Bound::Excluded(&e) => e,
                    std::ops::Bound::Unbounded => self.len(),
                }),
                std::ops::Bound::Unbounded => (0, self.len()),
            };

            self.erase_indices(start, end);
            let entries = self.entries.split_off(end);
            let drained = self.entries.split_off(start);

            let indices = (0..entries.len()).collect();
            (Self { indices, entries }, drained.into_iter())
        }
    }

    let mut map = TestIndexMap::new(vec![
        Bucket { key: 1, value: "a" },
        Bucket { key: 2, value: "b" },
        Bucket { key: 3, value: "c" },
        Bucket { key: 4, value: "d" },
    ]);

    let (new_map, drained) = map.split_splice(..);
    assert_eq!(drained.collect::<Vec<_>>().len(), 4);
    assert_eq!(new_map.len(), 0);
}

#[test]
fn test_split_splice_empty() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct TestIndexMap<K, V> {
        entries: Vec<Bucket<K, V>>,
        // Assuming we have an Indices type used for some form of indexing
        indices: Vec<usize> 
    }

    impl<K, V> TestIndexMap<K, V> {
        fn new(entries: Vec<Bucket<K, V>>) -> Self {
            let indices = (0..entries.len()).collect();
            Self { entries, indices }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn erase_indices(&mut self, start: usize, end: usize) {
            self.entries.drain(start..end);
            self.indices.drain(start..end);
        }

        fn split_splice<R>(&mut self, range: R) -> (Self, std::vec::IntoIter<Bucket<K, V>>)
        where
            R: std::ops::RangeBounds<usize>,
        {
            let (start, end) = match range.start_bound() {
                std::ops::Bound::Included(&s) => (s, match range.end_bound() {
                    std::ops::Bound::Included(&e) => e + 1,
                    std::ops::Bound::Excluded(&e) => e,
                    std::ops::Bound::Unbounded => self.len(),
                }),
                std::ops::Bound::Excluded(&s) => (s + 1, match range.end_bound() {
                    std::ops::Bound::Included(&e) => e + 1,
                    std::ops::Bound::Excluded(&e) => e,
                    std::ops::Bound::Unbounded => self.len(),
                }),
                std::ops::Bound::Unbounded => (0, self.len()),
            };

            self.erase_indices(start, end);
            let entries = self.entries.split_off(end);
            let drained = self.entries.split_off(start);

            let indices = (0..entries.len()).collect();
            (Self { indices, entries }, drained.into_iter())
        }
    }

    let mut map = TestIndexMap::new(vec![]);

    let (new_map, drained) = map.split_splice(0..1);
    assert_eq!(drained.collect::<Vec<_>>().len(), 0);
    assert_eq!(new_map.len(), 0);
}

