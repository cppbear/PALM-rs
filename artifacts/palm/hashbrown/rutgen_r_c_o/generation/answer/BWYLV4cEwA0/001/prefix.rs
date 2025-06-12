// Answer 0

#[test]
fn test_capacity_zero() {
    let map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::default(), Global);
    let cap = map.capacity();
}

#[test]
fn test_capacity_small() {
    let map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::default(), Global);
    let cap = map.capacity();
}

#[test]
fn test_capacity_medium() {
    let map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(100, DefaultHashBuilder::default(), Global);
    let cap = map.capacity();
}

#[test]
fn test_capacity_large() {
    let map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(1_000_000, DefaultHashBuilder::default(), Global);
    let cap = map.capacity();
}

#[test]
fn test_capacity_max() {
    let map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(1 << 30, DefaultHashBuilder::default(), Global);
    let cap = map.capacity();
}

#[should_panic]
fn test_capacity_exceed_limit() {
    let map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in((1 << 30) + 1, DefaultHashBuilder::default(), Global);
    let cap = map.capacity();
}

