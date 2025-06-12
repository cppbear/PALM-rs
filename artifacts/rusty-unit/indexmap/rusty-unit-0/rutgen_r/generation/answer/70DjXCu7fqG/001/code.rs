// Answer 0

#[test]
fn test_get_index_of_empty_map() {
    use indexmap::IndexMap;

    let map: IndexMap<String, i32> = IndexMap::new();
    let result = map.get_index_of(&"non_existent_key");
    assert_eq!(result, None);
}

