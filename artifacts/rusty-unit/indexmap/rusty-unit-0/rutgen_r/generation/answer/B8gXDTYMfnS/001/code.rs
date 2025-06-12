// Answer 0

#[test]
fn test_key_mut_valid_index() {
    struct Entry<K> {
        key: K,
    }

    struct Map<K> {
        entries: Vec<Entry<K>>,
        index: usize,
    }

    impl<K> Map<K> {
        pub fn new(entries: Vec<Entry<K>>, index: usize) -> Self {
            Map { entries, index }
        }

        pub fn key_mut(&mut self) -> &mut K {
            let index = self.index;
            &mut self.entries[index].key
        }
    }

    // Test case with valid index
    let mut map = Map::new(vec![Entry { key: 1 }, Entry { key: 2 }], 1);
    let key_ref = map.key_mut();
    *key_ref += 5; // Modify the value
    assert_eq!(*key_ref, 7);
}

#[test]
#[should_panic]
fn test_key_mut_invalid_index() {
    struct Entry<K> {
        key: K,
    }

    struct Map<K> {
        entries: Vec<Entry<K>>,
        index: usize,
    }

    impl<K> Map<K> {
        pub fn new(entries: Vec<Entry<K>>, index: usize) -> Self {
            Map { entries, index }
        }

        pub fn key_mut(&mut self) -> &mut K {
            let index = self.index;
            &mut self.entries[index].key
        }
    }

    // Test case with out-of-bounds index
    let mut map = Map::new(vec![Entry { key: 1 }, Entry { key: 2 }], 2);
    let _key_ref = map.key_mut(); // This should panic
}

