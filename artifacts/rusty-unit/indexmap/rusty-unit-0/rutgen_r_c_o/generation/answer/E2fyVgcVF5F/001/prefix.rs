// Answer 0

#[test]
fn test_get_valid_index() {
    let mut entries = Entries::new(); // Initialize Entries
    entries.push(Bucket { hash: HashValue::new(1), key: "key1", value: "value1" });
    entries.push(Bucket { hash: HashValue::new(2), key: "key2", value: "value2" });
    
    let mut map = IndexMapCore::new(); // Initialize IndexMapCore
    map.entries = entries;

    let indexed_entry = IndexedEntry::new(&mut map, 0);
    indexed_entry.get();
}

#[test]
fn test_get_second_entry() {
    let mut entries = Entries::new(); // Initialize Entries
    entries.push(Bucket { hash: HashValue::new(1), key: "key1", value: "value1" });
    entries.push(Bucket { hash: HashValue::new(2), key: "key2", value: "value2" });
    
    let mut map = IndexMapCore::new(); // Initialize IndexMapCore
    map.entries = entries;

    let indexed_entry = IndexedEntry::new(&mut map, 1);
    indexed_entry.get();
}

#[test]
#[should_panic]
fn test_get_out_of_bounds_index() {
    let mut entries = Entries::new(); // Initialize Entries
    entries.push(Bucket { hash: HashValue::new(1), key: "key1", value: "value1" });
    
    let mut map = IndexMapCore::new(); // Initialize IndexMapCore
    map.entries = entries;

    let indexed_entry = IndexedEntry::new(&mut map, 1); // Index 1 is out of bounds
    indexed_entry.get();
}

#[test]
fn test_get_empty_map() {
    let mut entries = Entries::new(); // Initialize Entries
    let mut map = IndexMapCore::new(); // Initialize IndexMapCore
    map.entries = entries; 

    let indexed_entry = IndexedEntry::new(&mut map, 0); // Index 0 on an empty map
    indexed_entry.get(); // This should panic
}

