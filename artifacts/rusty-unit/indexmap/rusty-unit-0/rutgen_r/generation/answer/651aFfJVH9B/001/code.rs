// Answer 0

#[test]
fn test_index_within_bounds() {
    struct MockEntry<V> {
        value: V,
    }

    struct MockMap<V> {
        entries: Vec<MockEntry<V>>,
    }

    impl<V> MockMap<V> {
        fn new(entries: Vec<MockEntry<V>>) -> Self {
            Self { entries }
        }

        fn index(&self, index: usize) -> &V {
            &self.entries[index].value
        }
    }

    let entries = vec![
        MockEntry { value: 10 },
        MockEntry { value: 20 },
        MockEntry { value: 30 },
    ];
    let mock_map = MockMap::new(entries);

    assert_eq!(*mock_map.index(0), 10);
    assert_eq!(*mock_map.index(1), 20);
    assert_eq!(*mock_map.index(2), 30);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_index_out_of_bounds_lower() {
    struct MockEntry<V> {
        value: V,
    }

    struct MockMap<V> {
        entries: Vec<MockEntry<V>>,
    }

    impl<V> MockMap<V> {
        fn new(entries: Vec<MockEntry<V>>) -> Self {
            Self { entries }
        }

        fn index(&self, index: usize) -> &V {
            &self.entries[index].value
        }
    }

    let entries = vec![MockEntry { value: 10 }];
    let mock_map = MockMap::new(entries);

    // Trigger panic by accessing an out-of-bounds index
    let _ = mock_map.index(1);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_index_out_of_bounds_upper() {
    struct MockEntry<V> {
        value: V,
    }

    struct MockMap<V> {
        entries: Vec<MockEntry<V>>,
    }

    impl<V> MockMap<V> {
        fn new(entries: Vec<MockEntry<V>>) -> Self {
            Self { entries }
        }

        fn index(&self, index: usize) -> &V {
            &self.entries[index].value
        }
    }

    let entries = vec![MockEntry { value: 10 }];
    let mock_map = MockMap::new(entries);

    // Trigger panic by accessing an index larger than the maximum
    let _ = mock_map.index(2);
}

