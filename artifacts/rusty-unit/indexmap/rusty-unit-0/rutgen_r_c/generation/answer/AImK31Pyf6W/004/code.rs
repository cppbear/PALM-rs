// Answer 0

#[test]
fn test_shift_remove_full_empty() {
    struct DummyKey;
    struct DummyValue;

    let mut index_map: IndexMap<DummyKey, DummyValue, RandomState> = IndexMap::new();
    assert_eq!(index_map.shift_remove_full(&DummyKey), None);
}

#[test]
fn test_shift_remove_full_one_element_not_matching() {
    struct DummyKey;
    struct DummyValue;

    let mut index_map: IndexMap<DummyKey, DummyValue, RandomState> = IndexMap::new();
    index_map.insert(DummyKey, DummyValue);
    assert_eq!(index_map.shift_remove_full(&DummyKey), None);
}

#[test]
fn test_shift_remove_full_multiple_elements_non_matching_key() {
    struct DummyKey;
    struct DummyValue;

    let mut index_map: IndexMap<DummyKey, DummyValue, RandomState> = IndexMap::new();
    index_map.insert(DummyKey, DummyValue);
    index_map.insert(DummyKey, DummyValue);

    assert_eq!(index_map.shift_remove_full(&DummyKey), None);
}

