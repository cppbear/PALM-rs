// Answer 0

#[test]
fn test_as_entries_empty() {
    let map: IndexMapCore<usize, usize> = IndexMapCore {
        indices: hash_table::HashTable::default(),
        entries: Vec::new(),
    };
    let _entries = map.as_entries();
}

#[test]
fn test_as_entries_single_entry() {
    let map: IndexMapCore<usize, usize> = IndexMapCore {
        indices: hash_table::HashTable::default(),
        entries: vec![Bucket {
            hash: HashValue(1),
            key: 1,
            value: 100,
        }],
    };
    let _entries = map.as_entries();
}

#[test]
fn test_as_entries_multiple_entries() {
    let map: IndexMapCore<usize, usize> = IndexMapCore {
        indices: hash_table::HashTable::default(),
        entries: vec![
            Bucket { hash: HashValue(1), key: 1, value: 100 },
            Bucket { hash: HashValue(2), key: 2, value: 200 },
            Bucket { hash: HashValue(3), key: 3, value: 300 },
        ],
    };
    let _entries = map.as_entries();
}

#[test]
fn test_as_entries_large_number_of_entries() {
    let entries: Vec<Bucket<usize, usize>> = (0..1000)
        .map(|i| Bucket { hash: HashValue(i as u64), key: i, value: i * 10 })
        .collect();
    let map: IndexMapCore<usize, usize> = IndexMapCore {
        indices: hash_table::HashTable::default(),
        entries,
    };
    let _entries = map.as_entries();
}

