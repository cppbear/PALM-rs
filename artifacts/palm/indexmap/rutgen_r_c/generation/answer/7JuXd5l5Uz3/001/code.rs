// Answer 0

#[test]
fn test_len_empty_map() {
    struct TestIndexMap<K, V> {
        core: IndexMapCore<K, V>,
    }

    let map: TestIndexMap<i32, i32> = TestIndexMap {
        core: IndexMapCore::new(),
    };
    
    assert_eq!(map.len(), 0);
}

#[test]
fn test_len_non_empty_map() {
    struct TestIndexMap<K, V> {
        core: IndexMapCore<K, V>,
    }

    let mut map: TestIndexMap<i32, i32> = TestIndexMap {
        core: IndexMapCore::with_capacity(5),
    };

    let entries = vec![(1, 10), (2, 20), (3, 30)];
    map.core.entries.extend(entries);

    assert_eq!(map.len(), 3);
}

#[test]
fn test_len_after_clearing_map() {
    struct TestIndexMap<K, V> {
        core: IndexMapCore<K, V>,
    }

    let mut map: TestIndexMap<i32, i32> = TestIndexMap {
        core: IndexMapCore::with_capacity(5),
    };

    let entries = vec![(1, 10), (2, 20)];
    map.core.entries.extend(entries);

    map.core.clear();

    assert_eq!(map.len(), 0);
}

#[test]
fn test_len_after_trim_map() {
    struct TestIndexMap<K, V> {
        core: IndexMapCore<K, V>,
    }

    let mut map: TestIndexMap<i32, i32> = TestIndexMap {
        core: IndexMapCore::with_capacity(5),
    };

    let entries = vec![(1, 10), (2, 20), (3, 30), (4, 40)];
    map.core.entries.extend(entries);
    
    map.core.truncate(2);

    assert_eq!(map.len(), 2);
}

