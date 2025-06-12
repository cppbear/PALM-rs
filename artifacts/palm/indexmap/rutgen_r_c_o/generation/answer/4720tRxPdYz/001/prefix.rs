// Answer 0

#[test]
fn test_swap_remove_entry_valid_index() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    let entry = IndexedEntry::new(&mut map, 0);
    entry.swap_remove_entry();
}

#[test]
fn test_swap_remove_entry_last_index() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    let entry = IndexedEntry::new(&mut map, 0);
    entry.swap_remove_entry();
}

#[test]
#[should_panic]
fn test_swap_remove_entry_out_of_bounds() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    let entry = IndexedEntry::new(&mut map, 1);
    entry.swap_remove_entry();
}

#[test]
fn test_swap_remove_entry_middle_index() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");
    let entry = IndexedEntry::new(&mut map, 1);
    entry.swap_remove_entry();
}

#[test]
fn test_swap_remove_entry_multiple_removals() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");
    
    let entry1 = IndexedEntry::new(&mut map, 1);
    entry1.swap_remove_entry();
    
    let entry2 = IndexedEntry::new(&mut map, 0);
    entry2.swap_remove_entry();
}

