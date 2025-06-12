// Answer 0

#[test]
fn test_index_mut_valid_key() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(1, "value1".to_string());
    let value = map.index_mut(&1);
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_mut_key_not_found() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(2, "value2".to_string());
    let value = map.index_mut(&3);
}

#[test]
fn test_index_mut_multiple_entries() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(0, "zero".to_string());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    let value_zero = map.index_mut(&0);
    let value_one = map.index_mut(&1);
    let value_two = map.index_mut(&2);
}

#[test]
fn test_index_mut_edge_case_first_entry() {
    let mut map: IndexMap<u32, String, RandomState> = IndexMap::new();
    map.insert(100, "hundred".to_string());
    let value = map.index_mut(&100);
}

#[test]
fn test_index_mut_edge_case_last_entry() {
    let mut map: IndexMap<u32, String, RandomState> = IndexMap::new();
    map.insert(200, "two hundred".to_string());
    let value = map.index_mut(&200);
}

#[test]
fn test_index_mut_no_entries() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    // This should panic since the map is empty
    let value = map.index_mut(&1);
}

