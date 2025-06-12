// Answer 0

#[test]
fn test_last_entry_empty_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    let entry = map.last_entry();
}

#[test]
fn test_last_entry_single_element_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    let entry = map.last_entry();
}

