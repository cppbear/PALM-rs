// Answer 0

#[test]
fn test_swap_remove_basic() {
    let mut map = IndexMapCore::new();
    map.insert("key1", 1);
    let entry = OccupiedEntry::new(&mut map, map.get_occupied_entry("key1").unwrap());
    entry.swap_remove();
}

#[test]
fn test_swap_remove_multiple_entries() {
    let mut map = IndexMapCore::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    let entry = OccupiedEntry::new(&mut map, map.get_occupied_entry("key2").unwrap());
    entry.swap_remove();
}

#[test]
fn test_swap_remove_first_entry() {
    let mut map = IndexMapCore::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    let entry = OccupiedEntry::new(&mut map, map.get_occupied_entry("key1").unwrap());
    entry.swap_remove();
}

#[test]
fn test_swap_remove_last_entry() {
    let mut map = IndexMapCore::new();
    map.insert("key1", 1);
    let entry = OccupiedEntry::new(&mut map, map.get_occupied_entry("key1").unwrap());
    entry.swap_remove();
}

#[test]
fn test_swap_remove_edge_case() {
    let mut map = IndexMapCore::new();
    for i in 1..=10 {
        map.insert(format!("key{}", i).as_str(), i);
    }
    let entry = OccupiedEntry::new(&mut map, map.get_occupied_entry("key5").unwrap());
    entry.swap_remove();
}

