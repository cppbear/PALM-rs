// Answer 0

#[test]
fn test_first_mut_with_non_empty_map() {
    struct TestBucket<K, V> {
        key: K,
        value: V,
    }

    struct TestMap<K, V> {
        entries: Vec<TestBucket<K, V>>,
    }

    impl<K, V> TestMap<K, V> {
        pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
            self.entries.first_mut().map(|entry| (&entry.key, &mut entry.value))
        }
    }

    let mut map = TestMap {
        entries: vec![
            TestBucket { key: "first", value: 1 },
            TestBucket { key: "second", value: 2 },
        ],
    };

    if let Some((key, value)) = map.first_mut() {
        assert_eq!(*key, "first");
        *value += 1;
    }

    assert_eq!(map.entries[0].value, 2);
}

#[test]
fn test_first_mut_with_empty_map() {
    struct TestBucket<K, V> {
        key: K,
        value: V,
    }

    struct TestMap<K, V> {
        entries: Vec<TestBucket<K, V>>,
    }

    impl<K, V> TestMap<K, V> {
        pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
            self.entries.first_mut().map(|entry| (&entry.key, &mut entry.value))
        }
    }

    let mut map: TestMap<&str, i32> = TestMap { entries: Vec::new() };

    assert_eq!(map.first_mut(), None);
}

