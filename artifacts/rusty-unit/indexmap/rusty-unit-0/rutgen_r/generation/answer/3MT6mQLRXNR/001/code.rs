// Answer 0

#[test]
fn test_get_index_mut_valid_index() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct Map<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> Map<K, V> {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }

        fn push(&mut self, key: K, value: V) {
            self.entries.push(Bucket { key, value });
        }

        pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
            self.entries.get_mut(index).map(|bucket| (&bucket.key, &mut bucket.value))
        }
    }

    let mut map = Map::new();
    map.push("a", 1);
    map.push("b", 2);
    
    if let Some((key, value)) = map.get_index_mut(0) {
        assert_eq!(*key, "a");
        *value += 1;
    }
    assert_eq!(map.get_index_mut(0).unwrap().1, &2);
}

#[test]
fn test_get_index_mut_invalid_index_too_high() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct Map<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> Map<K, V> {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }

        fn push(&mut self, key: K, value: V) {
            self.entries.push(Bucket { key, value });
        }

        pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
            self.entries.get_mut(index).map(|bucket| (&bucket.key, &mut bucket.value))
        }
    }

    let mut map = Map::new();
    map.push("a", 1);
    
    assert!(map.get_index_mut(1).is_none());
}

#[test]
fn test_get_index_mut_invalid_index_negative() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct Map<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> Map<K, V> {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }

        fn push(&mut self, key: K, value: V) {
            self.entries.push(Bucket { key, value });
        }

        pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
            self.entries.get_mut(index).map(|bucket| (&bucket.key, &mut bucket.value))
        }
    }

    let mut map = Map::new();
    map.push("a", 1);
    
    // Rust does not allow negative indexing, hence we won't create a test for negative index.
    // However, this test verifies normal behavior without panicking
    assert!(map.get_index_mut(usize::MAX).is_none());
}

