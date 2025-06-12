// Answer 0

#[test]
fn test_get_index_of_with_empty_map() {
    let map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    let key = &1;
    map.get_index_of(key);
}

#[test]
fn test_get_index_of_with_one_entry_matching() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    let key = &1;
    map.get_index_of(key);
}

#[test]
fn test_get_index_of_with_one_entry_not_matching() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    let key = &2;
    map.get_index_of(key);
}

#[test]
fn test_get_index_of_with_multiple_entries() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let key = &2;
    map.get_index_of(key);
}

#[test]
fn test_get_index_of_with_not_present_key_in_multiple_entries() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let key = &4;
    map.get_index_of(key);
}

#[test]
fn test_get_index_of_with_last_entry_matching() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let key = &3;
    map.get_index_of(key);
}

