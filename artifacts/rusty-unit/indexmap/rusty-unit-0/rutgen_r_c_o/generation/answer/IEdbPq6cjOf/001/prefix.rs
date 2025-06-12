// Answer 0

#[test]
fn test_insert_valid_entry() {
    let mut map = IndexMapCore::new();
    map.insert("key1", 1);
    let mut entry = IndexedEntry::new(&mut map, 0);
    let old_value = entry.insert(2);
}

#[test]
fn test_insert_different_value() {
    let mut map = IndexMapCore::new();
    map.insert("key1", 1);
    let mut entry = IndexedEntry::new(&mut map, 0);
    let old_value = entry.insert(3);
}

#[test]
fn test_insert_with_edge_case_index() {
    let mut map = IndexMapCore::new();
    map.insert("key1", 1);
    let mut entry = IndexedEntry::new(&mut map, 0);
    let old_value = entry.insert(4);
}

#[test]
#[should_panic]
fn test_insert_same_value() {
    let mut map = IndexMapCore::new();
    map.insert("key1", 1);
    let mut entry = IndexedEntry::new(&mut map, 0);
    let old_value = entry.insert(1);
}

#[test]
fn test_insert_multiple_entries() {
    let mut map = IndexMapCore::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    let mut entry1 = IndexedEntry::new(&mut map, 0);
    let old_value1 = entry1.insert(3);
    let mut entry2 = IndexedEntry::new(&mut map, 1);
    let old_value2 = entry2.insert(4);
}

