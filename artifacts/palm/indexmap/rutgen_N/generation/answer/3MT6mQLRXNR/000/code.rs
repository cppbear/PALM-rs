// Answer 0

#[test]
fn test_get_index_mut_valid_index() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct IndexMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> IndexMap<K, V> {
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

        fn len(&self) -> usize {
            self.entries.len()
        }
    }

    let mut map = IndexMap::new();
    map.push("a", 1);
    map.push("b", 2);

    if let Some((key, value)) = map.get_index_mut(1) {
        assert_eq!(*key, "b");
        *value += 1;
    }
    
    assert_eq!(map.get_index_mut(1).unwrap().1, &3);
}

#[test]
fn test_get_index_mut_invalid_index() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct IndexMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> IndexMap<K, V> {
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

        fn len(&self) -> usize {
            self.entries.len()
        }
    }

    let mut map = IndexMap::new();
    map.push("a", 1);
    map.push("b", 2);

    assert!(map.get_index_mut(2).is_none());
}

