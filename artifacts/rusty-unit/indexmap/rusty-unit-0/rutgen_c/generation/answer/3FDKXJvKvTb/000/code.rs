// Answer 0

#[test]
fn test_swap_indices_valid() {
    struct TestKey(usize);
    struct TestValue(i32);
    struct TestIndices {
        data: Vec<usize>,
    }
    
    struct TestEntries<K, V> {
        keys: Vec<K>,
        values: Vec<V>,
    }
    
    struct TestIndexMapCore<K, V> {
        indices: TestIndices,
        entries: TestEntries<K, V>,
    }
    
    impl<K, V> TestIndexMapCore<K, V> {
        fn borrow_mut(&mut self) -> RefMut<K, V> {
            RefMut::new(&mut self.indices, &mut self.entries)
        }
    }
    
    let mut indices = TestIndices { data: vec![0, 1, 2] };
    let mut entries = TestEntries { keys: vec![TestKey(0), TestKey(1), TestKey(2)], values: vec![TestValue(10), TestValue(20), TestValue(30)] };
    let mut map = TestIndexMapCore { indices, entries };

    let entry_1 = IndexedEntry::new(&mut map, 0);
    let mut entry_2 = IndexedEntry::new(&mut map, 1);

    entry_1.map.swap_indices(0, 1);

    assert_eq!(entry_1.map.entries.keys[0], TestKey(1));
    assert_eq!(entry_1.map.entries.keys[1], TestKey(0));
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds() {
    struct TestKey(usize);
    struct TestValue(i32);
    struct TestIndices {
        data: Vec<usize>,
    }
    
    struct TestEntries<K, V> {
        keys: Vec<K>,
        values: Vec<V>,
    }
    
    struct TestIndexMapCore<K, V> {
        indices: TestIndices,
        entries: TestEntries<K, V>,
    }
    
    impl<K, V> TestIndexMapCore<K, V> {
        fn borrow_mut(&mut self) -> RefMut<K, V> {
            RefMut::new(&mut self.indices, &mut self.entries)
        }
    }

    let mut indices = TestIndices { data: vec![0, 1, 2] };
    let mut entries = TestEntries { keys: vec![TestKey(0), TestKey(1), TestKey(2)], values: vec![TestValue(10), TestValue(20), TestValue(30)] };
    let mut map = TestIndexMapCore { indices, entries };

    let entry = IndexedEntry::new(&mut map, 0);
    
    entry.map.swap_indices(0, 3); // This should panic due to out of bounds index
}

