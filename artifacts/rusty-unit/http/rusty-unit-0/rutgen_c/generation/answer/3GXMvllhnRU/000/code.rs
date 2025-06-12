// Answer 0

#[test]
fn test_values_empty() {
    struct DummyHeaderValue;
    let map: HeaderMap<DummyHeaderValue> = HeaderMap::with_capacity(0);
    let values: Values<DummyHeaderValue> = map.values();
    let mut collected: Vec<_> = values.inner.collect();
    assert!(collected.is_empty());
}

#[test]
fn test_values_single_entry() {
    struct DummyHeaderValue;
    let mut map: HeaderMap<DummyHeaderValue> = HeaderMap::with_capacity(1);
    map.insert("TestKey", DummyHeaderValue);
    let values: Values<DummyHeaderValue> = map.values();
    let collected: Vec<_> = values.inner.collect();
    assert_eq!(collected.len(), 1);
}

#[test]
fn test_values_multiple_entries() {
    struct DummyHeaderValue;
    let mut map: HeaderMap<DummyHeaderValue> = HeaderMap::with_capacity(2);
    map.insert("Key1", DummyHeaderValue);
    map.append("Key2", DummyHeaderValue);
    let values: Values<DummyHeaderValue> = map.values();
    let collected: Vec<_> = values.inner.collect();
    assert_eq!(collected.len(), 2);
}

#[test]
fn test_values_after_clear() {
    struct DummyHeaderValue;
    let mut map: HeaderMap<DummyHeaderValue> = HeaderMap::with_capacity(2);
    map.insert("Key1", DummyHeaderValue);
    map.clear();
    let values: Values<DummyHeaderValue> = map.values();
    let collected: Vec<_> = values.inner.collect();
    assert!(collected.is_empty());
}

