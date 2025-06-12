// Answer 0

#[test]
fn test_clear_non_empty_map() {
    let mut map = HeaderMap::with_capacity(10);
    map.insert("Key1", "Value1");
    map.insert("Key2", "Value2");
    map.clear();
}

#[test]
fn test_clear_with_excessive_capacity() {
    let mut map = HeaderMap::try_with_capacity(20).unwrap();
    for i in 0..15 {
        map.insert(format!("Key{}", i), format!("Value{}", i));
    }
    map.clear();
}

#[test]
fn test_clear_edge_case_one_element() {
    let mut map = HeaderMap::with_capacity(5);
    map.insert("SingleKey", "SingleValue");
    map.clear();
}

#[test]
fn test_clear_large_capacity() {
    let mut map = HeaderMap::try_with_capacity(1 << 14).unwrap(); // just below max size
    for i in 0..(1 << 13) {
        map.insert(format!("Key{}", i), format!("Value{}", i));
    }
    map.clear();
}

#[test]
fn test_clear_empty_map() {
    let mut map = HeaderMap::with_capacity(0);
    map.clear(); // Should be a no-op
}

#[test]
fn test_clear_with_no_insertions() {
    let mut map = HeaderMap::with_capacity(1);
    map.clear(); // Ensure it clears without panic
}

