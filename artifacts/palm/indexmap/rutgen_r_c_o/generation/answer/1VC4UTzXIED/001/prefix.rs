// Answer 0

#[test]
fn test_binary_search_by_key_existing_key() {
    let mut index_map = IndexMap::new();
    index_map.insert(10, 20);
    index_map.insert(30, 40);
    index_map.insert(50, 60);
    let result = index_map.binary_search_by_key(&20, |k, v| *v);
}

#[test]
fn test_binary_search_by_key_non_existing_key_lower_bound() {
    let mut index_map = IndexMap::new();
    index_map.insert(20, 30);
    index_map.insert(40, 50);
    index_map.insert(60, 70);
    let result = index_map.binary_search_by_key(&25, |k, v| *v);
}

#[test]
fn test_binary_search_by_key_non_existing_key_upper_bound() {
    let mut index_map = IndexMap::new();
    index_map.insert(15, 25);
    index_map.insert(35, 45);
    index_map.insert(55, 65);
    let result = index_map.binary_search_by_key(&70, |k, v| *v);
}

#[test]
fn test_binary_search_by_key_exact_match() {
    let mut index_map = IndexMap::new();
    index_map.insert(0, 10);
    index_map.insert(1, 20);
    index_map.insert(2, 30);
    let result = index_map.binary_search_by_key(&20, |k, v| *v);
}

#[test]
fn test_binary_search_by_key_edge_case_empty() {
    let mut index_map: IndexMap<i32, i32> = IndexMap::new();
    // Expected to panic or return an error since the map is empty.
    let result = index_map.binary_search_by_key(&0, |k, v| *v);
}

#[test]
fn test_binary_search_by_key_single_element() {
    let mut index_map = IndexMap::new();
    index_map.insert(1, 1);
    let result = index_map.binary_search_by_key(&1, |k, v| *v);
}

#[test]
fn test_binary_search_by_key_with_negative() {
    let mut index_map = IndexMap::new();
    index_map.insert(-10, 0);
    index_map.insert(-5, 25);
    index_map.insert(0, 50);
    let result = index_map.binary_search_by_key(&-7, |k, v| *v);
}

