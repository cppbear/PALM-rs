// Answer 0

#[test]
fn test_remove_from_non_empty_map() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct MyMap<K, V> {
        entries: Vec<Entry<K, V>>,
    }

    impl<K, V> MyMap<K, V> {
        pub fn new() -> Self {
            Self { entries: Vec::new() }
        }

        pub fn add(&mut self, key: K, value: V) {
            self.entries.push(Entry { key, value });
        }

        pub fn swap_remove(&mut self, index: usize) -> V {
            self.entries.swap_remove(index).value
        }

        pub fn remove(&mut self, index: usize) -> V {
            self.swap_remove(index)
        }
    }

    let mut map = MyMap::new();
    map.add(1, "value1");
    map.add(2, "value2");

    let removed_value = map.remove(0);
    assert_eq!(removed_value, "value1");
}

#[test]
#[should_panic]
fn test_remove_from_empty_map() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct MyMap<K, V> {
        entries: Vec<Entry<K, V>>,
    }

    impl<K, V> MyMap<K, V> {
        pub fn new() -> Self {
            Self { entries: Vec::new() }
        }

        pub fn swap_remove(&mut self, index: usize) -> V {
            self.entries.swap_remove(index).value
        }

        pub fn remove(&mut self, index: usize) -> V {
            self.swap_remove(index)
        }
    }

    let mut map: MyMap<i32, &str> = MyMap::new();
    map.remove(0); // This will panic, as we're removing from an empty map.
}

#[test]
fn test_remove_from_single_element_map() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct MyMap<K, V> {
        entries: Vec<Entry<K, V>>,
    }

    impl<K, V> MyMap<K, V> {
        pub fn new() -> Self {
            Self { entries: Vec::new() }
        }

        pub fn add(&mut self, key: K, value: V) {
            self.entries.push(Entry { key, value });
        }

        pub fn swap_remove(&mut self, index: usize) -> V {
            self.entries.swap_remove(index).value
        }

        pub fn remove(&mut self, index: usize) -> V {
            self.swap_remove(index)
        }
    }

    let mut map = MyMap::new();
    map.add(1, "only_value");

    let removed_value = map.remove(0);
    assert_eq!(removed_value, "only_value");
    // Here we can verify the map is now empty
    assert!(map.entries.is_empty());
}

