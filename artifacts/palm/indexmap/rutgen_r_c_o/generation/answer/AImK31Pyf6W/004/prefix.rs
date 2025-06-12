// Answer 0

#[test]
fn test_shift_remove_full_empty() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    let key: &i32 = &1;
    map.shift_remove_full(key);
}

#[test]
fn test_shift_remove_full_single_element_not_found() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(2, 20);
    let key: &i32 = &1;
    map.shift_remove_full(key);
}

#[test]
fn test_shift_remove_full_single_element_found() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    let key: &i32 = &1;
    map.shift_remove_full(key);
}

