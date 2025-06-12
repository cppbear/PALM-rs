// Answer 0

#[test]
fn test_last_mut_exists() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> TestMap<K, V> {
        pub fn new() -> Self {
            Self { entries: Vec::new() }
        }

        pub fn push(&mut self, key: K, value: V) {
            self.entries.push(Bucket { key, value });
        }

        pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {
            self.entries.last_mut().map(|bucket| (&bucket.key, &mut bucket.value))
        }
    }

    let mut map = TestMap::new();
    map.push(1, 10);
    map.push(2, 20);

    if let Some((key, value)) = map.last_mut() {
        assert_eq!(*key, 2);
        *value += 5;
    }

    assert_eq!(map.last_mut().map(|(_, v)| *v), Some(25));
}

#[test]
fn test_last_mut_empty() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> TestMap<K, V> {
        pub fn new() -> Self {
            Self { entries: Vec::new() }
        }

        pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {
            self.entries.last_mut().map(|bucket| (&bucket.key, &mut bucket.value))
        }
    }

    let mut map: TestMap<i32, i32> = TestMap::new();
    assert_eq!(map.last_mut(), None);
}

