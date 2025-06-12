// Answer 0

#[test]
fn test_iter_empty() {
    struct TestMap<K, V> {
        inner: std::collections::HashMap<K, V>,
    }

    impl<K, V> TestMap<K, V> {
        pub fn new() -> Self {
            TestMap {
                inner: std::collections::HashMap::new(),
            }
        }

        pub fn iter(&self) -> std::iter::Iter<'_, K, V> {
            self.inner.iter()
        }
    }

    let map: TestMap<i32, i32> = TestMap::new();
    let iter = map.iter();
    assert_eq!(iter.count(), 0);
}

#[test]
fn test_iter_single_element() {
    struct TestMap<K, V> {
        inner: std::collections::HashMap<K, V>,
    }

    impl<K, V> TestMap<K, V> {
        pub fn new() -> Self {
            TestMap {
                inner: std::collections::HashMap::new(),
            }
        }

        pub fn insert(&mut self, key: K, value: V) {
            self.inner.insert(key, value);
        }

        pub fn iter(&self) -> std::iter::Iter<'_, K, V> {
            self.inner.iter()
        }
    }

    let mut map = TestMap::new();
    map.insert(1, 10);
    let iter = map.iter();
    assert_eq!(iter.count(), 1);
}

#[test]
fn test_iter_multiple_elements() {
    struct TestMap<K, V> {
        inner: std::collections::HashMap<K, V>,
    }

    impl<K, V> TestMap<K, V> {
        pub fn new() -> Self {
            TestMap {
                inner: std::collections::HashMap::new(),
            }
        }

        pub fn insert(&mut self, key: K, value: V) {
            self.inner.insert(key, value);
        }

        pub fn iter(&self) -> std::iter::Iter<'_, K, V> {
            self.inner.iter()
        }
    }

    let mut map = TestMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let iter = map.iter();
    assert_eq!(iter.count(), 3);
}

