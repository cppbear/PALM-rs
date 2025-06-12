// Answer 0

#[test]
fn test_values_empty_map() {
    struct Map<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> Map<K, V> {
        pub fn new() -> Self {
            Map { entries: Vec::new() }
        }

        pub fn values(&self) -> impl Iterator<Item = &V> {
            self.entries.iter().map(|(_, v)| v)
        }
    }

    let map: Map<i32, i32> = Map::new();
    let values: Vec<_> = map.values().collect();
    assert_eq!(values.len(), 0);
}

#[test]
fn test_values_single_entry() {
    struct Map<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> Map<K, V> {
        pub fn new() -> Self {
            Map { entries: Vec::new() }
        }

        pub fn insert(&mut self, key: K, value: V) {
            self.entries.push((key, value));
        }

        pub fn values(&self) -> impl Iterator<Item = &V> {
            self.entries.iter().map(|(_, v)| v)
        }
    }

    let mut map: Map<i32, i32> = Map::new();
    map.insert(1, 100);
    let values: Vec<_> = map.values().collect();
    assert_eq!(values.len(), 1);
    assert_eq!(*values[0], 100);
}

#[test]
fn test_values_multiple_entries() {
    struct Map<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> Map<K, V> {
        pub fn new() -> Self {
            Map { entries: Vec::new() }
        }

        pub fn insert(&mut self, key: K, value: V) {
            self.entries.push((key, value));
        }

        pub fn values(&self) -> impl Iterator<Item = &V> {
            self.entries.iter().map(|(_, v)| v)
        }
    }

    let mut map: Map<i32, i32> = Map::new();
    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);
    let values: Vec<_> = map.values().collect();
    assert_eq!(values.len(), 3);
    assert_eq!(*values[0], 100);
    assert_eq!(*values[1], 200);
    assert_eq!(*values[2], 300);
}

