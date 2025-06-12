// Answer 0

#[test]
fn test_last_empty_map() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> TestMap<K, V> {
        pub fn as_entries(&self) -> &[Bucket<K, V>] {
            &self.entries
        }
        
        pub fn last(&self) -> Option<(&K, &V)> {
            self.as_entries().last().map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    let map: TestMap<i32, i32> = TestMap { entries: vec![] };
    assert_eq!(map.last(), None);
}

#[test]
fn test_last_single_entry() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> TestMap<K, V> {
        pub fn as_entries(&self) -> &[Bucket<K, V>] {
            &self.entries
        }
        
        pub fn last(&self) -> Option<(&K, &V)> {
            self.as_entries().last().map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    let map = TestMap {
        entries: vec![Bucket {
            hash: 1.into(),
            key: 42,
            value: 100,
        }],
    };
    let result = map.last();
    assert!(result.is_some());
    assert_eq!(result.unwrap(), (&42, &100));
}

#[test]
fn test_last_multiple_entries() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> TestMap<K, V> {
        pub fn as_entries(&self) -> &[Bucket<K, V>] {
            &self.entries
        }
        
        pub fn last(&self) -> Option<(&K, &V)> {
            self.as_entries().last().map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    let map = TestMap {
        entries: vec![
            Bucket {
                hash: 1.into(),
                key: 1,
                value: 10,
            },
            Bucket {
                hash: 2.into(),
                key: 2,
                value: 20,
            },
            Bucket {
                hash: 3.into(),
                key: 3,
                value: 30,
            },
        ],
    };
    let result = map.last();
    assert!(result.is_some());
    assert_eq!(result.unwrap(), (&3, &30));
}

#[test]
fn test_last_with_different_types() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> TestMap<K, V> {
        pub fn as_entries(&self) -> &[Bucket<K, V>] {
            &self.entries
        }
        
        pub fn last(&self) -> Option<(&K, &V)> {
            self.as_entries().last().map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    let map = TestMap {
        entries: vec![Bucket {
            hash: 1.into(),
            key: "last_key",
            value: "last_value",
        }],
    };
    let result = map.last();
    assert!(result.is_some());
    assert_eq!(result.unwrap(), (&"last_key", &"last_value"));
}

