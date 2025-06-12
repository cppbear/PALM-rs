// Answer 0

#[test]
fn test_sort_by() {
    struct TestKey(i32);
    struct TestValue(i32);
    
    let mut map = IndexMap::new();
    map.insert(TestKey(3), TestValue(30));
    map.insert(TestKey(1), TestValue(10));
    map.insert(TestKey(2), TestValue(20));

    map.sort_by(|k1, v1, k2, v2| {
        if k1.0 == k2.0 {
            v1.0.cmp(&v2.0)
        } else {
            k1.0.cmp(&k2.0)
        }
    });

    let entries: Vec<_> = map.into_entries();
    assert_eq!(entries[0].key.0, 1);
    assert_eq!(entries[1].key.0, 2);
    assert_eq!(entries[2].key.0, 3);
}

#[test]
fn test_sort_by_same_keys() {
    struct TestKey(i32);
    struct TestValue(i32);

    let mut map = IndexMap::new();
    map.insert(TestKey(1), TestValue(30));
    map.insert(TestKey(1), TestValue(10));
    map.insert(TestKey(1), TestValue(20));

    map.sort_by(|k1, v1, k2, v2| {
        v1.0.cmp(&v2.0)
    });

    let entries: Vec<_> = map.into_entries();
    assert_eq!(entries[0].value.0, 10);
    assert_eq!(entries[1].value.0, 20);
    assert_eq!(entries[2].value.0, 30);
}

#[test]
fn test_sort_by_empty() {
    struct TestKey(i32);
    struct TestValue(i32);

    let mut map: IndexMap<TestKey, TestValue> = IndexMap::new();
    
    map.sort_by(|k1, v1, k2, v2| k1.0.cmp(&k2.0)); // Just to ensure it runs without panic

    assert!(map.is_empty());
}

