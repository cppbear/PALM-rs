// Answer 0

#[test]
fn test_partition_point_basic() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct SortedMap<K, V> {
        entries: Vec<Entry<K, V>>,
    }

    impl<K, V> SortedMap<K, V> {
        pub fn partition_point<P>(&self, mut pred: P) -> usize
        where
            P: FnMut(&K, &V) -> bool,
        {
            self.entries
                .iter()
                .position(|entry| !pred(&entry.key, &entry.value))
                .unwrap_or(self.entries.len())
        }
    }

    let map = SortedMap {
        entries: vec![
            Entry { key: 1, value: "a" },
            Entry { key: 2, value: "b" },
            Entry { key: 3, value: "c" },
            Entry { key: 4, value: "d" },
        ],
    };

    let result = map.partition_point(|&key, &_value| key < 3);
    assert_eq!(result, 2);
}

#[test]
fn test_partition_point_empty() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct SortedMap<K, V> {
        entries: Vec<Entry<K, V>>,
    }

    impl<K, V> SortedMap<K, V> {
        pub fn partition_point<P>(&self, mut pred: P) -> usize
        where
            P: FnMut(&K, &V) -> bool,
        {
            self.entries
                .iter()
                .position(|entry| !pred(&entry.key, &entry.value))
                .unwrap_or(self.entries.len())
        }
    }

    let map: SortedMap<i32, &str> = SortedMap { entries: Vec::new() };

    let result = map.partition_point(|&key, &_value| key < 3);
    assert_eq!(result, 0);
}

#[test]
fn test_partition_point_all_true() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct SortedMap<K, V> {
        entries: Vec<Entry<K, V>>,
    }

    impl<K, V> SortedMap<K, V> {
        pub fn partition_point<P>(&self, mut pred: P) -> usize
        where
            P: FnMut(&K, &V) -> bool,
        {
            self.entries
                .iter()
                .position(|entry| !pred(&entry.key, &entry.value))
                .unwrap_or(self.entries.len())
        }
    }

    let map = SortedMap {
        entries: vec![
            Entry { key: 1, value: "a" },
            Entry { key: 2, value: "b" },
            Entry { key: 3, value: "c" },
        ],
    };

    let result = map.partition_point(|&key, &_value| key < 4);
    assert_eq!(result, 3);
}

#[test]
fn test_partition_point_all_false() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct SortedMap<K, V> {
        entries: Vec<Entry<K, V>>,
    }

    impl<K, V> SortedMap<K, V> {
        pub fn partition_point<P>(&self, mut pred: P) -> usize
        where
            P: FnMut(&K, &V) -> bool,
        {
            self.entries
                .iter()
                .position(|entry| !pred(&entry.key, &entry.value))
                .unwrap_or(self.entries.len())
        }
    }

    let map = SortedMap {
        entries: vec![
            Entry { key: 1, value: "a" },
            Entry { key: 2, value: "b" },
            Entry { key: 3, value: "c" },
        ],
    };

    let result = map.partition_point(|&key, &_value| key > 3);
    assert_eq!(result, 0);
}

