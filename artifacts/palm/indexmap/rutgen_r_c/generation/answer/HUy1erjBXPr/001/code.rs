// Answer 0

#[test]
fn test_into_entries_non_empty() {
    let entries = vec![
        Bucket { hash: HashValue::from_raw(1), key: 1, value: "a" },
        Bucket { hash: HashValue::from_raw(2), key: 2, value: "b" },
    ];
    let core = IndexMapCore {
        indices: hash_table::HashTable::new(),
        entries,
    };
    let result = core.into_entries();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].key, 1);
    assert_eq!(result[0].value, "a");
}

#[test]
fn test_into_entries_empty() {
    let core = IndexMapCore {
        indices: hash_table::HashTable::new(),
        entries: vec![],
    };
    let result = core.into_entries();
    assert!(result.is_empty());
}

#[test]
fn test_into_entries_with_duplicate_keys() {
    let entries = vec![
        Bucket { hash: HashValue::from_raw(1), key: 1, value: "a" },
        Bucket { hash: HashValue::from_raw(1), key: 1, value: "b" },
    ];
    let core = IndexMapCore {
        indices: hash_table::HashTable::new(),
        entries,
    };
    let result = core.into_entries();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].value, "a");
    assert_eq!(result[1].value, "b");
}

