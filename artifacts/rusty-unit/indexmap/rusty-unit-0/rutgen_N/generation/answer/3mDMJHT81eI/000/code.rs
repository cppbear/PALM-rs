// Answer 0

#[test]
fn test_values_mut() {
    struct TestMapSlice<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMapSlice<K, V> {
        fn new() -> Self {
            TestMapSlice { entries: Vec::new() }
        }

        fn insert(&mut self, key: K, value: V) {
            self.entries.push((key, value));
        }
        
        pub fn values_mut(&mut self) -> Vec<&mut V> {
            self.entries.iter_mut().map(|(_, value)| value).collect()
        }
    }

    let mut map_slice = TestMapSlice::new();
    map_slice.insert(1, "a");
    map_slice.insert(2, "b");

    let values = map_slice.values_mut();
    
    values[0] = &mut "c";
    values[1] = &mut "d";

    assert_eq!(map_slice.entries[0].1, "c");
    assert_eq!(map_slice.entries[1].1, "d");
}

#[test]
fn test_values_mut_empty() {
    struct TestMapSlice<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMapSlice<K, V> {
        fn new() -> Self {
            TestMapSlice { entries: Vec::new() }
        }
        
        pub fn values_mut(&mut self) -> Vec<&mut V> {
            self.entries.iter_mut().map(|(_, value)| value).collect()
        }
    }

    let mut map_slice = TestMapSlice::new();
    let values = map_slice.values_mut();
    
    assert!(values.is_empty());
}

