// Answer 0

#[test]
fn test_remove_existing_key() {
    struct TestMap<K, V> {
        inner: IndexMap<K, V, RandomState>,
    }

    let mut map = TestMap {
        inner: IndexMap::new(),
    };

    map.inner.insert("key1", "value1");
    map.inner.insert("key2", "value2");

    let removed_value = map.inner.remove(&"key1");

    assert_eq!(removed_value, Some("value1"));
    assert_eq!(map.inner.get(&"key1"), None);
}

#[test]
fn test_remove_non_existing_key() {
    struct TestMap<K, V> {
        inner: IndexMap<K, V, RandomState>,
    }

    let mut map = TestMap {
        inner: IndexMap::new(),
    };

    map.inner.insert("key1", "value1");

    let removed_value = map.inner.remove(&"key2");

    assert_eq!(removed_value, None);
}

#[test]
fn test_remove_key_from_empty_map() {
    struct TestMap<K, V> {
        inner: IndexMap<K, V, RandomState>,
    }

    let mut map = TestMap {
        inner: IndexMap::new(),
    };

    let removed_value = map.inner.remove(&"key1");

    assert_eq!(removed_value, None);
}

