// Answer 0

#[test]
fn test_first_with_non_empty_map() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }
    
    struct Bucket<K, V> {
        key: K,
        value: V,
    }
    
    impl<K, V> TestMap<K, V> {
        pub fn first(&self) -> Option<(&K, &V)> {
            self.entries.first().map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    let map = TestMap {
        entries: vec![
            Bucket { key: 1, value: "a" },
            Bucket { key: 2, value: "b" },
        ],
    };

    assert_eq!(map.first(), Some((&1, &"a")));
}

#[test]
fn test_first_with_empty_map() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }
    
    struct Bucket<K, V> {
        key: K,
        value: V,
    }
    
    impl<K, V> TestMap<K, V> {
        pub fn first(&self) -> Option<(&K, &V)> {
            self.entries.first().map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    let map = TestMap { entries: vec![] };

    assert_eq!(map.first(), None);
}

