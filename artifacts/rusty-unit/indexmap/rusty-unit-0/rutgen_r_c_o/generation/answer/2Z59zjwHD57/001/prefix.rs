// Answer 0

#[test]
fn test_remove_with_valid_entry() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::new();
    map.insert(0, 10);
    map.insert(1, 20);
    let index_entry = map.entries().occupied_entry(0).unwrap();
    index_entry.remove();
}

#[test]
fn test_remove_with_last_entry() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::new();
    map.insert(0, 30);
    let index_entry = map.entries().occupied_entry(0).unwrap();
    index_entry.remove();
}

#[test]
#[should_panic]
fn test_remove_with_non_existent_entry() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::new();
    map.insert(0, 40);
    map.remove(&0);
    let index_entry = map.entries().occupied_entry(0).unwrap();  // Should panic here
    index_entry.remove();
}

#[test]
fn test_remove_with_multiple_entries() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::new();
    for i in 0..5 {
        map.insert(i, i * 10);
    }
    let index_entry = map.entries().occupied_entry(2).unwrap();
    index_entry.remove();
}

#[test]
fn test_remove_with_edge_case_high_value() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::new();
    map.insert(0, 100);
    let index_entry = map.entries().occupied_entry(0).unwrap();
    index_entry.remove();
}

#[test]
fn test_remove_with_edge_case_low_value() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::new();
    map.insert(0, 0);
    let index_entry = map.entries().occupied_entry(0).unwrap();
    index_entry.remove();
}

