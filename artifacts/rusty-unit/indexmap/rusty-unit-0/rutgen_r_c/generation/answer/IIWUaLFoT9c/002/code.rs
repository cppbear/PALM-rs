// Answer 0

#[test]
fn test_get_range_valid_range() {
    struct TestIndexSet {
        entries: Vec<Bucket<i32, ()>>,
    }

    impl TestIndexSet {
        fn as_entries(&self) -> &[Bucket<i32, ()>] {
            &self.entries
        }
        
        fn len(&self) -> usize {
            self.entries.len()
        }
    }

    let index_set = TestIndexSet {
        entries: vec![
            Bucket { hash: 0, key: 1, value: () },
            Bucket { hash: 0, key: 2, value: () },
            Bucket { hash: 0, key: 3, value: () },
        ],
    };

    let result = index_set.get_range(0..2);
    assert!(result.is_some());
    let slice = result.unwrap();
    assert_eq!(slice.entries.len(), 2);
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[1].key, 2);
}

#[test]
fn test_get_range_boundary_start_inclusive_end_exclusive() {
    struct TestIndexSet {
        entries: Vec<Bucket<i32, ()>>,
    }

    impl TestIndexSet {
        fn as_entries(&self) -> &[Bucket<i32, ()>] {
            &self.entries
        }
        
        fn len(&self) -> usize {
            self.entries.len()
        }
    }

    let index_set = TestIndexSet {
        entries: vec![
            Bucket { hash: 0, key: 1, value: () },
            Bucket { hash: 0, key: 2, value: () },
            Bucket { hash: 0, key: 3, value: () },
        ],
    };

    let result = index_set.get_range(..=2);
    assert!(result.is_some());
    let slice = result.unwrap();
    assert_eq!(slice.entries.len(), 3);
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[1].key, 2);
    assert_eq!(slice.entries[2].key, 3);
}

#[test]
fn test_get_range_out_of_bounds() {
    struct TestIndexSet {
        entries: Vec<Bucket<i32, ()>>,
    }

    impl TestIndexSet {
        fn as_entries(&self) -> &[Bucket<i32, ()>] {
            &self.entries
        }

        fn len(&self) -> usize {
            self.entries.len()
        }
    }

    let index_set = TestIndexSet {
        entries: vec![
            Bucket { hash: 0, key: 1, value: () },
            Bucket { hash: 0, key: 2, value: () },
            Bucket { hash: 0, key: 3, value: () },
        ],
    };

    let result = index_set.get_range(10..20);
    assert!(result.is_none());
}

#[test]
fn test_get_range_empty() {
    struct TestIndexSet {
        entries: Vec<Bucket<i32, ()>>,
    }

    impl TestIndexSet {
        fn as_entries(&self) -> &[Bucket<i32, ()>] {
            &self.entries
        }

        fn len(&self) -> usize {
            self.entries.len()
        }
    }

    let index_set = TestIndexSet { entries: vec![] };

    let result = index_set.get_range(0..1);
    assert!(result.is_none());
}

