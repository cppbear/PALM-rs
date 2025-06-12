// Answer 0

#[test]
fn test_header_map_new_empty() {
    let map = HeaderMap::new();
}

#[test]
fn test_header_map_new_capacity_zero() {
    let map_zero_capacity = HeaderMap::try_with_capacity(0).unwrap();
}

