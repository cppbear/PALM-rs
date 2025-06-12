// Answer 0

#[test]
fn test_indexed_entry_new() {
    struct TestKey;
    struct TestValue;

    let mut index_map = IndexMapCore::new();
    let index = 0; // Testing with the first index

    let entry = IndexedEntry::new(&mut index_map, index);

    assert_eq!(entry.index, index);
    // Note: The `map` field cannot be directly tested for equality as it is a reference.
}

#[test]
fn test_indexed_entry_new_with_capacity() {
    struct TestKey;
    struct TestValue;

    let mut index_map = IndexMapCore::with_capacity(5);
    let index = 0; // Testing with the first index

    let entry = IndexedEntry::new(&mut index_map, index);

    assert_eq!(entry.index, index);
    // Note: The `map` field cannot be directly tested for equality as it is a reference.
}

