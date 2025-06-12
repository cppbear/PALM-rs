// Answer 0

#[test]
fn test_shift_remove_index_valid_cases() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    index_map.insert(0, 10);
    index_map.insert(1, 20);
    index_map.insert(2, 30);
    
    let _ = index_map.shift_remove_index(0);
    let _ = index_map.shift_remove_index(1);
    let _ = index_map.shift_remove_index(0);
}

#[test]
fn test_shift_remove_index_empty() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    let _ = index_map.shift_remove_index(0);
}

#[test]
#[should_panic]
fn test_shift_remove_index_out_of_bounds() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    index_map.insert(0, 10);
    let _ = index_map.shift_remove_index(1);
}

#[test]
fn test_shift_remove_index_single_element() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    index_map.insert(0, 10);
    let _ = index_map.shift_remove_index(0);
}

#[test]
fn test_shift_remove_index_multiple_elements() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    index_map.insert(0, 10);
    index_map.insert(1, 20);
    index_map.insert(2, 30);
    index_map.insert(3, 40);

    let _ = index_map.shift_remove_index(2);
}

