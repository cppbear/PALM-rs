// Answer 0

#[test]
fn test_index_map_core_new() {
    let map: IndexMapCore<usize, String> = IndexMapCore::new();
}

#[test]
fn test_index_map_core_new_with_different_types() {
    let map: IndexMapCore<String, f64> = IndexMapCore::new();
}

#[test]
fn test_index_map_core_new_empty() {
    let map: IndexMapCore<i32, i32> = IndexMapCore::new();
}

