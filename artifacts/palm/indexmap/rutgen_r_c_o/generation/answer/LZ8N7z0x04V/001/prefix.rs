// Answer 0

#[test]
fn test_index_valid_key() {
    let mut map: IndexMap<u32, String, RandomState> = IndexMap::default();
    map.insert(1, "Value 1".to_string());
    map.insert(2, "Value 2".to_string());
    map.insert(3, "Value 3".to_string());
    
    let _value = map.index(&2);
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_non_existent_key() {
    let mut map: IndexMap<u32, String, RandomState> = IndexMap::default();
    map.insert(1, "Value 1".to_string());

    let _value = map.index(&3);
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_empty_map() {
    let map: IndexMap<u32, String, RandomState> = IndexMap::default();
    
    let _value = map.index(&1);
} 

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_negative_key() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::default();
    map.insert(1, "Value 1".to_string());

    let _value = map.index(&-1);
} 

#[test]
fn test_index_edge_large_key() {
    let mut map: IndexMap<u32, String, RandomState> = IndexMap::default();
    map.insert(u32::MAX, "Max Value".to_string());
    
    let _value = map.index(&u32::MAX);
}

