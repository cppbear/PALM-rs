// Answer 0

#[test]
fn test_new_empty_map() {
    let map = Map::new();
}

#[test]
fn test_new_with_capacity_zero() {
    let map = Map::with_capacity(0);
}

#[test]
fn test_new_with_capacity_max() {
    let map = Map::with_capacity(100);
}

#[test]
fn test_new_with_capacity_mid() {
    let map = Map::with_capacity(50);
}

