// Answer 0

#[test]
fn test_insert_sorted_key_found_update_value() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    
    let (index, old_value) = map.insert_sorted(2, "updated two".to_string());
}

#[test]
fn test_insert_sorted_key_found_update_value_at_start() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    
    map.insert(1, "one".to_string());
    
    let (index, old_value) = map.insert_sorted(1, "updated one".to_string());
}

#[test]
fn test_insert_sorted_key_found_update_value_at_end() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    
    let (index, old_value) = map.insert_sorted(2, "updated two".to_string());
}

#[test]
fn test_insert_sorted_key_not_found_insert_new_value() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    
    map.insert(1, "one".to_string());
    map.insert(3, "three".to_string());
    
    let (index, old_value) = map.insert_sorted(2, "two".to_string());
}

#[test]
fn test_insert_sorted_key_not_found_insert_new_value_at_start() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    
    map.insert(1, "one".to_string());
    
    let (index, old_value) = map.insert_sorted(0, "zero".to_string());
}

#[test]
fn test_insert_sorted_key_not_found_insert_new_value_at_end() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    
    let (index, old_value) = map.insert_sorted(4, "four".to_string());
}

