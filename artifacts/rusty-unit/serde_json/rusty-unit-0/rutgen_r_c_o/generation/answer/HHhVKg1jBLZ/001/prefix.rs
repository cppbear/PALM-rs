// Answer 0

#[test]
fn test_with_capacity_zero() {
    let _map = Map::with_capacity(0);
}

#[test]
fn test_with_capacity_one() {
    let _map = Map::with_capacity(1);
}

#[test]
fn test_with_capacity_ten() {
    let _map = Map::with_capacity(10);
}

#[test]
fn test_with_capacity_one_hundred() {
    let _map = Map::with_capacity(100);
}

#[test]
fn test_with_capacity_max() {
    let _map = Map::with_capacity(usize::MAX);
}

