// Answer 0

#[derive(Debug, PartialEq)]
struct Entry<K, V> {
    key: K,
    value: V,
}

#[derive(Debug)]
struct Map<K, V> {
    entries: Vec<Entry<K, V>>,
}

impl<K, V> Map<K, V> {
    pub fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Vec<Entry<K, V>>),
    {
        f(&mut self.entries);
    }

    pub fn sort_unstable_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> std::cmp::Ordering,
    {
        self.with_entries(move |entries| {
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
        });
    }
}

#[test]
fn test_sort_unstable_by_keys() {
    let mut map = Map {
        entries: vec![
            Entry { key: 3, value: "three" },
            Entry { key: 1, value: "one" },
            Entry { key: 2, value: "two" },
        ],
    };

    map.sort_unstable_by(|k1, v1, k2, v2| k1.cmp(k2));
    
    assert_eq!(map.entries, vec![
        Entry { key: 1, value: "one" },
        Entry { key: 2, value: "two" },
        Entry { key: 3, value: "three" },
    ]);
}

#[test]
fn test_sort_unstable_by_values() {
    let mut map = Map {
        entries: vec![
            Entry { key: 1, value: "banana" },
            Entry { key: 2, value: "apple" },
            Entry { key: 3, value: "cherry" },
        ],
    };

    map.sort_unstable_by(|k1, v1, k2, v2| v1.cmp(v2));
    
    assert_eq!(map.entries, vec![
        Entry { key: 2, value: "apple" },
        Entry { key: 1, value: "banana" },
        Entry { key: 3, value: "cherry" },
    ]);
}

#[test]
fn test_sort_unstable_with_equal_keys() {
    let mut map = Map {
        entries: vec![
            Entry { key: 1, value: "a" },
            Entry { key: 1, value: "b" },
            Entry { key: 2, value: "c" },
        ],
    };

    map.sort_unstable_by(|k1, v1, k2, v2| k1.cmp(k2));
    
    // Check that the order of equal keys is not preserved
    assert_eq!(map.entries.len(), 3);
    assert!(map.entries[0].key <= map.entries[1].key);
    assert!(map.entries[1].key <= map.entries[2].key);
}

#[test]
fn test_empty_map_sort() {
    let mut map: Map<i32, &str> = Map { entries: vec![] };

    map.sort_unstable_by(|_, _, _, _| std::cmp::Ordering::Equal);
    
    assert!(map.entries.is_empty());
}

