// Answer 0

#[test]
fn test_first_non_empty() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        pub fn first(&self) -> Option<(&K, &V)> {
            self.entries.first().map(|entry| (&entry.0, &entry.1))
        }
    }

    let map: TestMap<i32, &str> = TestMap {
        entries: vec![(1, "one"), (2, "two"), (3, "three")],
    };

    assert_eq!(map.first(), Some((&1, &"one")));
}

#[test]
fn test_first_empty() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        pub fn first(&self) -> Option<(&K, &V)> {
            self.entries.first().map(|entry| (&entry.0, &entry.1))
        }
    }

    let map: TestMap<i32, &str> = TestMap { entries: vec![] };

    assert_eq!(map.first(), None);
}

