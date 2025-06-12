// Answer 0

#[test]
fn test_map_with_capacity_zero() {
    let map = serde_json::Map::with_capacity(0);
    assert_eq!(map.len(), 0);
}

#[test]
fn test_map_with_capacity_non_zero() {
    let map = serde_json::Map::with_capacity(5);
    assert_eq!(map.len(), 0);
}

