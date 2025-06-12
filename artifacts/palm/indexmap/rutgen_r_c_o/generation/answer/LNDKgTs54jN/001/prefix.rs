// Answer 0

#[test]
fn test_shift_insert_vacant_entry_at_len() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    map.shift_insert(map.len(), '*', ());
}

#[test]
fn test_shift_insert_vacant_entry_with_different_key() {
    let mut map: IndexMap<i32, i32> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.shift_insert(map.len(), 1, 100);
}

#[test]
fn test_shift_insert_vacant_entry_with_string_key() {
    let mut map: IndexMap<String, i32> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.shift_insert(map.len(), String::from("key"), 42);
}

#[test]
fn test_shift_insert_vacant_entry_in_empty_map() {
    let mut map: IndexMap<char, ()> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    map.shift_insert(0, 'A', ());
}

#[test]
fn test_shift_insert_vacant_entry_after_multiple_insertions() {
    let mut map: IndexMap<u32, u32> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.shift_insert(0, 1, 10);
    map.shift_insert(1, 2, 20);
    map.shift_insert(map.len(), 3, 30);
}

