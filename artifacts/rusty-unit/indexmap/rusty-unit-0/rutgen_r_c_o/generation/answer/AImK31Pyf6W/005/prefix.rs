// Answer 0

#[test]
fn test_shift_remove_full_case_multiple_entries() {
    let mut index_map = IndexMap::with_capacity(5);
    index_map.insert(1, "one");
    index_map.insert(2, "two");
    index_map.insert(3, "three");
    
    let result = index_map.shift_remove_full(&2);
}

#[test]
fn test_shift_remove_full_case_single_entry() {
    let mut index_map = IndexMap::new();
    index_map.insert(1, "one");
    
    let result = index_map.shift_remove_full(&1);
}

#[test]
fn test_shift_remove_full_case_empty_map() {
    let mut index_map: IndexMap<i32, &str> = IndexMap::new();
    
    let result = index_map.shift_remove_full(&1);
}

#[test]
fn test_shift_remove_full_case_key_not_found() {
    let mut index_map = IndexMap::with_capacity(5);
    index_map.insert(1, "one");
    index_map.insert(2, "two");
    index_map.insert(3, "three");
    
    let result = index_map.shift_remove_full(&4);
}

