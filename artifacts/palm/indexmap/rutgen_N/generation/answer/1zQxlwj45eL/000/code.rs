// Answer 0

#[test]
fn test_shift_insert_with_valid_index() {
    struct TestEntry<V> {
        map: TestMap<V>,
        hash: u64,
        key: usize,
    }

    struct TestMap<V> {
        entries: Vec<TestEntryValue<V>>,
    }

    struct TestEntryValue<V> {
        value: V,
    }

    impl<V> TestMap<V> {
        fn new() -> Self {
            TestMap {
                entries: Vec::new(),
            }
        }
        
        fn shift_insert_unique(&mut self, index: usize, _hash: u64, _key: usize, value: V) {
            if index > self.entries.len() {
                panic!("Index out of bounds");
            }
            self.entries.insert(index, TestEntryValue { value });
        }
    }

    let mut entry = TestEntry {
        map: TestMap::new(),
        hash: 0,
        key: 0,
    };

    let value_ref = entry.shift_insert(0, 42);
    assert_eq!(*value_ref, 42);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_shift_insert_with_out_of_bounds_index() {
    struct TestEntry<V> {
        map: TestMap<V>,
        hash: u64,
        key: usize,
    }

    struct TestMap<V> {
        entries: Vec<TestEntryValue<V>>,
    }

    struct TestEntryValue<V> {
        value: V,
    }

    impl<V> TestMap<V> {
        fn new() -> Self {
            TestMap {
                entries: Vec::new(),
            }
        }
        
        fn shift_insert_unique(&mut self, index: usize, _hash: u64, _key: usize, _: V) {
            if index > self.entries.len() {
                panic!("Index out of bounds");
            }
            // Implementation omitted for brevity
        }
    }

    let mut entry = TestEntry {
        map: TestMap::new(),
        hash: 0,
        key: 0,
    };

    entry.shift_insert(1, 42);  // This should panic
}

