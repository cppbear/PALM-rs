// Answer 0

#[test]
fn test_index_valid_key() {
    struct TestIndexMap<K, V> {
        map: IndexMap<K, V, RandomState>,
    }

    let mut test_map = TestIndexMap {
        map: IndexMap::new(),
    };

    test_map.map.insert("key1", "value1");
    
    let result = test_map.map.index(&"key1");
    assert_eq!(result, &"value1");
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_invalid_key() {
    struct TestIndexMap<K, V> {
        map: IndexMap<K, V, RandomState>,
    }

    let mut test_map = TestIndexMap {
        map: IndexMap::new(),
    };

    test_map.map.insert("key1", "value1");

    // This should panic as "key2" does not exist
    let _ = test_map.map.index(&"key2");
}

