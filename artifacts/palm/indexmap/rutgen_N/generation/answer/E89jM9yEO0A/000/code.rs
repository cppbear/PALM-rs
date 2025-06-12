// Answer 0

#[test]
fn test_last_with_non_empty_map() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    impl<K, V> TestMap<K, V> {
        pub fn last(&self) -> Option<(&K, &V)> {
            self.entries.last().map(|b| (&b.key, &b.value))
        }
    }

    let map = TestMap {
        entries: vec![
            Bucket { key: 1, value: "a" },
            Bucket { key: 2, value: "b" },
        ],
    };

    let last_entry = map.last();
    assert_eq!(last_entry, Some((&2, &"b")));
}

#[test]
fn test_last_with_empty_map() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    impl<K, V> TestMap<K, V> {
        pub fn last(&self) -> Option<(&K, &V)> {
            self.entries.last().map(|b| (&b.key, &b.value))
        }
    }

    let map: TestMap<i32, &str> = TestMap { entries: vec![] };

    let last_entry = map.last();
    assert_eq!(last_entry, None);
}

