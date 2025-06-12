// Answer 0

#[derive(Debug)]
struct DummyEntry<K, V> {
    key: K,
    value: V,
}

struct DummyMapSlice<K, V> {
    entries: Vec<DummyEntry<K, V>>,
}

impl<K, V> DummyMapSlice<K, V> {
    pub fn keys(&self) -> Vec<&K> {
        self.entries.iter().map(|entry| &entry.key).collect()
    }
}

#[test]
fn test_keys_with_empty_map_slice() {
    let map_slice: DummyMapSlice<i32, i32> = DummyMapSlice { entries: Vec::new() };
    let keys: Vec<&i32> = map_slice.keys();
    assert!(keys.is_empty());
}

#[test]
fn test_keys_with_single_entry() {
    let map_slice = DummyMapSlice {
        entries: vec![DummyEntry { key: 1, value: 100 }],
    };
    let keys: Vec<&i32> = map_slice.keys();
    assert_eq!(keys, vec![&1]);
}

#[test]
fn test_keys_with_multiple_entries() {
    let map_slice = DummyMapSlice {
        entries: vec![
            DummyEntry { key: 1, value: 100 },
            DummyEntry { key: 2, value: 200 },
            DummyEntry { key: 3, value: 300 },
        ],
    };
    let keys: Vec<&i32> = map_slice.keys();
    assert_eq!(keys, vec![&1, &2, &3]);
}

#[test]
fn test_keys_with_repeated_keys() {
    let map_slice = DummyMapSlice {
        entries: vec![
            DummyEntry { key: 1, value: 100 },
            DummyEntry { key: 1, value: 200 },
        ],
    };
    let keys: Vec<&i32> = map_slice.keys();
    assert_eq!(keys, vec![&1, &1]); // demonstration of repeated keys
}

