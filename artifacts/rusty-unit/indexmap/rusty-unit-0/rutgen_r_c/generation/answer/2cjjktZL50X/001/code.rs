// Answer 0

#[test]
fn test_indexed_entry_new_valid() {
    struct TestKey;
    struct TestValue;

    let mut index_map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    let index = 0; // Valid index for a new entry

    let entry = IndexedEntry::new(&mut index_map, index);
    
    assert_eq!(entry.index, index);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_indexed_entry_new_invalid_index() {
    struct TestKey;
    struct TestValue;

    let mut index_map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    let index = 1; // Invalid index since no entries exist

    let _entry = IndexedEntry::new(&mut index_map, index);
}

#[test]
fn test_indexed_entry_new_multiple_entries() {
    struct TestKey;
    struct TestValue;

    let mut index_map: IndexMapCore<TestKey, TestValue> = IndexMapCore::with_capacity(2);
    // Simulating insertion by incrementing the entries count directly for the sake of the test.
    index_map.entries.push(TestValue);
    index_map.entries.push(TestValue);

    let index = 1; // Valid index for an existing entry

    let entry = IndexedEntry::new(&mut index_map, index);
    
    assert_eq!(entry.index, index);
}

