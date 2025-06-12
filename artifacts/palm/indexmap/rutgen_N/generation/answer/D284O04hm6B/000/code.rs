// Answer 0

#[test]
fn test_index_mut_valid_index() {
    struct TestEntry<V> {
        value: V,
    }

    struct TestMap<V> {
        entries: Vec<TestEntry<V>>,
    }

    impl<V> TestMap<V> {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }

        fn push(&mut self, value: V) {
            self.entries.push(TestEntry { value });
        }

        fn index_mut(&mut self, index: usize) -> &mut V {
            &mut self.entries[index].value
        }
    }

    let mut map = TestMap::new();
    map.push(10);
    map.push(20);
    
    let value = map.index_mut(1);
    *value += 5;

    assert_eq!(map.entries[1].value, 25);
}

#[test]
#[should_panic]
fn test_index_mut_out_of_bounds() {
    struct TestEntry<V> {
        value: V,
    }

    struct TestMap<V> {
        entries: Vec<TestEntry<V>>,
    }

    impl<V> TestMap<V> {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }

        fn index_mut(&mut self, index: usize) -> &mut V {
            &mut self.entries[index].value
        }
    }

    let mut map = TestMap::new();
    map.push(10);

    // This should panic since there is only one entry (index 0)
    let _ = map.index_mut(1);
}

