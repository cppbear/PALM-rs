// Answer 0

#[test]
fn test_push_entry_initially_empty() {
    let mut map: IndexMapCore<isize, isize> = IndexMapCore::new();
    let hash_value = HashValue(1);
    map.push_entry(hash_value, 42, 100);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_push_entry_with_capacity() {
    let mut map: IndexMapCore<isize, isize> = IndexMapCore::with_capacity(2);
    let hash_value1 = HashValue(1);
    let hash_value2 = HashValue(2);
    map.push_entry(hash_value1, 42, 100);
    map.push_entry(hash_value2, 43, 200);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_push_entry_exceeding_capacity_triggers_reserve() {
    let mut map: IndexMapCore<isize, isize> = IndexMapCore::with_capacity(1);
    let hash_value1 = HashValue(1);
    let hash_value2 = HashValue(2);
    map.push_entry(hash_value1, 42, 100);
    map.push_entry(hash_value2, 43, 200); // Should trigger reserve
    assert_eq!(map.len(), 2);
}

#[test]
fn test_push_entry_multiple_entries() {
    let mut map: IndexMapCore<usize, &str> = IndexMapCore::new();
    for i in 0..5 {
        map.push_entry(HashValue(i), i, "value");
    }
    assert_eq!(map.len(), 5);
}

