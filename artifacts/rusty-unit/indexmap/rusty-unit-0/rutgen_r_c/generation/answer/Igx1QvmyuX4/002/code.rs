// Answer 0

#[test]
fn test_get_range_valid_range() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl TestMap {
        fn new(entries: Vec<(i32, String)>) -> Self {
            let mut buckets = Vec::new();
            for (key, value) in entries {
                buckets.push(Bucket {
                    hash: HashValue::from(key), // Assuming HashValue can be created from i32
                    key,
                    value,
                });
            }
            Self { entries: buckets }
        }

        fn as_entries(&self) -> &[Bucket<i32, String>] {
            &self.entries
        }

        fn get_range(&self, range: std::ops::Range<usize>) -> Option<&Slice<i32, String>> {
            let entries = self.as_entries();
            let len = entries.len();
            let start = range.start.min(len);
            let end = range.end.min(len);
            if start >= end {
                return None;
            }
            Some(&Slice { entries: (&entries[start..end]).to_vec() })
        }
    }

    let map = TestMap::new(vec![(1, "a".to_string()), (2, "b".to_string()), (3, "c".to_string())]);
    let slice = map.get_range(0..2).unwrap();
    assert_eq!(slice.entries.len(), 2);
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[1].key, 2);
}

#[test]
fn test_get_range_empty_range() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl TestMap {
        fn new(entries: Vec<(i32, String)>) -> Self {
            let mut buckets = Vec::new();
            for (key, value) in entries {
                buckets.push(Bucket {
                    hash: HashValue::from(key),
                    key,
                    value,
                });
            }
            Self { entries: buckets }
        }

        fn as_entries(&self) -> &[Bucket<i32, String>] {
            &self.entries
        }

        fn get_range(&self, range: std::ops::Range<usize>) -> Option<&Slice<i32, String>> {
            let entries = self.as_entries();
            let len = entries.len();
            let start = range.start.min(len);
            let end = range.end.min(len);
            
            if start >= end {
                return None;
            }
            Some(&Slice { entries: (&entries[start..end]).to_vec() })
        }
    }

    let map = TestMap::new(vec![(1, "a".to_string()), (2, "b".to_string())]);
    let slice = map.get_range(0..0);
    assert!(slice.is_none());
}

#[test]
fn test_get_range_full_range() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl TestMap {
        fn new(entries: Vec<(i32, String)>) -> Self {
            let mut buckets = Vec::new();
            for (key, value) in entries {
                buckets.push(Bucket {
                    hash: HashValue::from(key),
                    key,
                    value,
                });
            }
            Self { entries: buckets }
        }

        fn as_entries(&self) -> &[Bucket<i32, String>] {
            &self.entries
        }

        fn get_range(&self, range: std::ops::Range<usize>) -> Option<&Slice<i32, String>> {
            let entries = self.as_entries();
            let len = entries.len();
            let start = range.start.min(len);
            let end = range.end.min(len);
            
            if start >= end {
                return None;
            }
            Some(&Slice { entries: (&entries[start..end]).to_vec() })
        }
    }

    let map = TestMap::new(vec![(1, "a".to_string()), (2, "b".to_string()), (3, "c".to_string())]);
    let slice = map.get_range(0..3).unwrap();
    assert_eq!(slice.entries.len(), 3);
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[1].key, 2);
    assert_eq!(slice.entries[2].key, 3);
}

