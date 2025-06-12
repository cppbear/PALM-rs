// Answer 0

#[test]
fn swap_remove_full_empty_map() {
    struct TestKey;
    struct TestValue;

    let mut map: IndexMap<TestKey, TestValue, RandomState> = IndexMap::new();
    let key = TestKey;

    let result = map.swap_remove_full(&key);
    assert!(result.is_none());
}

#[test]
fn swap_remove_full_single_element_map_not_matching() {
    struct TestKey;
    struct TestValue;

    let mut map: IndexMap<TestKey, TestValue, RandomState> = IndexMap::new();
    let key1 = TestKey;
    let key2 = TestKey; // Distinct key, to ensure no match

    map.insert_full(HashValue(0), key1, TestValue);

    let result = map.swap_remove_full(&key2);
    assert!(result.is_none());
}

#[test]
fn swap_remove_full_single_element_map_matching() {
    struct TestKey;
    struct TestValue;

    let mut map: IndexMap<TestKey, TestValue, RandomState> = IndexMap::new();
    let key1 = TestKey;

    map.insert_full(HashValue(0), key1, TestValue);

    let result = map.swap_remove_full(&key1);
    assert!(result.is_some());
}

#[test]
fn swap_remove_full_two_elements_map_first() {
    struct TestKey;
    struct TestValue;

    let mut map: IndexMap<TestKey, TestValue, RandomState> = IndexMap::new();
    let key1 = TestKey;
    let key2 = TestKey; // Second distinct key 

    map.insert_full(HashValue(0), key1, TestValue);
    map.insert_full(HashValue(1), key2, TestValue);

    let result = map.swap_remove_full(&key1);
    assert!(result.is_some());
}

#[test]
fn swap_remove_full_two_elements_map_second() {
    struct TestKey;
    struct TestValue;

    let mut map: IndexMap<TestKey, TestValue, RandomState> = IndexMap::new();
    let key1 = TestKey;
    let key2 = TestKey; // Second distinct key 

    map.insert_full(HashValue(0), key1, TestValue);
    map.insert_full(HashValue(1), key2, TestValue);

    let result = map.swap_remove_full(&key2);
    assert!(result.is_some());
}

