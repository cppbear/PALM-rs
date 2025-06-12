// Answer 0

#[test]
fn test_into_entries_empty() {
    struct TestIndexMap {
        core: IndexMapCore<u32, String>,
    }

    let map: TestIndexMap = TestIndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
    };

    let entries: Vec<Bucket<u32, String>> = map.into_entries();
    assert!(entries.is_empty());
}

#[test]
fn test_into_entries_single() {
    struct TestIndexMap {
        core: IndexMapCore<u32, String>,
    }

    let mut map: TestIndexMap = TestIndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
    };

    map.core.entries.push(Bucket { hash: HashValue::new(), key: 1, value: "one".to_string() });

    let entries: Vec<Bucket<u32, String>> = map.into_entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[0].value, "one");
}

#[test]
fn test_into_entries_multiple() {
    struct TestIndexMap {
        core: IndexMapCore<u32, String>,
    }

    let mut map: TestIndexMap = TestIndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
    };

    map.core.entries.push(Bucket { hash: HashValue::new(), key: 1, value: "one".to_string() });
    map.core.entries.push(Bucket { hash: HashValue::new(), key: 2, value: "two".to_string() });

    let entries: Vec<Bucket<u32, String>> = map.into_entries();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[0].value, "one");
    assert_eq!(entries[1].key, 2);
    assert_eq!(entries[1].value, "two");
}

