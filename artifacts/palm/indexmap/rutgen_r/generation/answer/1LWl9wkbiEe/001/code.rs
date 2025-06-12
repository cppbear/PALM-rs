// Answer 0

#[test]
fn test_get_index_valid() {
    struct Bucket<'a, K, V> {
        key: &'a K,
        value: &'a V,
    }
    
    struct Slice<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> Slice<K, V> {
        pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
            self.entries.get(index).map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    let key1 = "a";
    let value1 = 1;
    let key2 = "b";
    let value2 = 2;
    let slice = Slice {
        entries: vec![
            Bucket { key: &key1, value: &value1 },
            Bucket { key: &key2, value: &value2 },
        ],
    };

    assert_eq!(slice.get_index(0), Some((&"a", &1)));
    assert_eq!(slice.get_index(1), Some((&"b", &2)));
}

#[test]
fn test_get_index_out_of_bounds() {
    struct Bucket<'a, K, V> {
        key: &'a K,
        value: &'a V,
    }
    
    struct Slice<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> Slice<K, V> {
        pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
            self.entries.get(index).map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    let slice = Slice { entries: Vec::new() };

    assert_eq!(slice.get_index(0), None);
    assert_eq!(slice.get_index(1), None);
}

#[test]
#[should_panic]
fn test_get_index_invalid_index() {
    struct Bucket<'a, K, V> {
        key: &'a K,
        value: &'a V,
    }
    
    struct Slice<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> Slice<K, V> {
        pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
            self.entries.get(index).map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    let key = "a";
    let value = 1;
    let slice = Slice {
        entries: vec![Bucket { key: &key, value: &value }],
    };

    // Using an out-of-bounds index
    let _ = slice.get_index(1).unwrap(); // This will not panic in the current implementation since it safely returns None
}

