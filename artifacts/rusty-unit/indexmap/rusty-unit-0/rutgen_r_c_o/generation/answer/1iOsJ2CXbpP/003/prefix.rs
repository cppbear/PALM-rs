// Answer 0

#[test]
fn test_swap_remove_full_empty() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    let key: i32 = 1;
    map.swap_remove_full(&key);
}

#[test]
fn test_swap_remove_full_single_element_non_matching() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    let key: i32 = 2;
    map.swap_remove_full(&key);
}

#[test]
fn test_swap_remove_full_single_element_matching() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    let key: i32 = 1;
    map.swap_remove_full(&key);
}

#[test]
fn test_swap_remove_full_multiple_elements_non_matching() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let key: i32 = 3;
    map.swap_remove_full(&key);
}

