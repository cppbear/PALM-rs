// Answer 0

#[test]
fn test_last_empty_index_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    let result = map.last();
}

#[test]
fn test_last_single_element_index_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    let result = map.last();
}

#[test]
fn test_last_multiple_elements_index_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let result = map.last();
}

#[test]
fn test_last_multiple_elements_index_map_large() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    for i in 1..=100 {
        map.insert(i, i * 10);
    }
    let result = map.last();
}

#[test]
fn test_last_after_removal() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.remove(&2);
    let result = map.last();
}

#[test]
fn test_last_with_duplicates() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(1, 15); // Duplicate key, should update
    map.insert(2, 20);
    let result = map.last();
}

