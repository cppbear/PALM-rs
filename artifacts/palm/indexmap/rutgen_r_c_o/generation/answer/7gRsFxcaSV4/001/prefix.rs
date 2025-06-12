// Answer 0

#[test]
fn test_first_entry_empty_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    let entry = map.first_entry();
}

#[test]
fn test_first_entry_single_element() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    let entry = map.first_entry();
}

#[test]
fn test_first_entry_multiple_elements() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let entry = map.first_entry();
}

#[test]
fn test_first_entry_large_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    for i in 1..=1000 {
        map.insert(i, i * 10);
    }
    let entry = map.first_entry();
}

#[test]
#[should_panic]
fn test_first_entry_out_of_bounds() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    let entry = map.get_index_entry(0);
}

