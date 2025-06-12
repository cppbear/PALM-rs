// Answer 0

#[test]
fn test_with_capacity_zero() {
    let map_core = IndexMapCore::with_capacity(0);
    assert_eq!(map_core.entries.len(), 0);
    assert_eq!(map_core.indices.len(), 0);
}

#[test]
fn test_with_capacity_positive() {
    let capacity = 10;
    let map_core = IndexMapCore::with_capacity(capacity);
    assert_eq!(map_core.entries.len(), 0);
    assert_eq!(map_core.indices.len(), 0);
}

