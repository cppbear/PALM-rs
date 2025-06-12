// Answer 0

#[test]
fn test_binary_search_keys_existing_key() {
    let mut map: IndexMap<i32, &str, RandomState> = IndexMap::new();
    map.insert(1, "one");
    map.insert(3, "three");
    map.insert(5, "five");

    // Existing key
    let result = map.binary_search_keys(&3);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_keys_non_existing_key() {
    let mut map: IndexMap<i32, &str, RandomState> = IndexMap::new();
    map.insert(1, "one");
    map.insert(3, "three");
    map.insert(5, "five");

    // Non-existing key
    let result = map.binary_search_keys(&4);
    assert_eq!(result, Err(2));
}

#[test]
fn test_binary_search_keys_lower_bound() {
    let mut map: IndexMap<i32, &str, RandomState> = IndexMap::new();
    map.insert(2, "two");
    map.insert(4, "four");
    map.insert(6, "six");

    // Value less than the lowest key
    let result = map.binary_search_keys(&1);
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_keys_upper_bound() {
    let mut map: IndexMap<i32, &str, RandomState> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    // Value greater than the highest key
    let result = map.binary_search_keys(&4);
    assert_eq!(result, Err(3));
}

#[test]
fn test_binary_search_keys_empty_map() {
    let map: IndexMap<i32, &str, RandomState> = IndexMap::new();

    // Searching in an empty map
    let result = map.binary_search_keys(&1);
    assert_eq!(result, Err(0));
}

