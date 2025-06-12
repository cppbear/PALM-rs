// Answer 0

#[test]
fn test_remove_entry_existing_entry() {
    let mut map = hashbrown::HashMap::new();
    map.insert("apple", 10);
    if let hashbrown::hash_map::Entry::Occupied(o) = map.entry("apple") {
        let result = o.remove_entry();
    }
}

#[test]
fn test_remove_entry_non_existing_entry() {
    let mut map = hashbrown::HashMap::new();
    if let hashbrown::hash_map::Entry::Occupied(o) = map.entry("banana") {
        let result = o.remove_entry();
    }
}

#[test]
fn test_remove_entry_multiple_entries() {
    let mut map = hashbrown::HashMap::new();
    map.insert("carrot", 20);
    map.insert("date", 30);
    if let hashbrown::hash_map::Entry::Occupied(o) = map.entry("carrot") {
        let result = o.remove_entry();
    }
    if let hashbrown::hash_map::Entry::Occupied(o) = map.entry("date") {
        let result = o.remove_entry();
    }
}

#[test]
#[should_panic]
fn test_remove_entry_empty_map() {
    let map: hashbrown::HashMap<&str, u32> = hashbrown::HashMap::new();
    if let hashbrown::hash_map::Entry::Occupied(o) = map.entry("fig") {
        let result = o.remove_entry();
    }
}

#[test]
fn test_remove_entry_after_insertion() {
    let mut map = hashbrown::HashMap::with_capacity(10);
    map.insert("grape", 40);
    if let hashbrown::hash_map::Entry::Occupied(o) = map.entry("grape") {
        let result = o.remove_entry();
    }
    assert!(map.is_empty());
}

#[test]
fn test_remove_entry_from_full_map() {
    let mut map = hashbrown::HashMap::with_capacity(100);
    for i in 1..=100 {
        map.insert(i, i * 100);
    }
    if let hashbrown::hash_map::Entry::Occupied(o) = map.entry(50) {
        let result = o.remove_entry();
    }
    assert_eq!(map.len(), 99);
}

