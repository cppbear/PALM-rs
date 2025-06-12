// Answer 0

#[test]
fn test_with_capacity_and_hasher_in_zero_capacity() {
    let s = DefaultHashBuilder::default();
    let alloc = Global;
    let map = HashMap::with_capacity_and_hasher_in(0, s, alloc);
}

#[test]
fn test_with_capacity_and_hasher_in_min_capacity() {
    let s = DefaultHashBuilder::default();
    let alloc = Global;
    let map = HashMap::with_capacity_and_hasher_in(1, s, alloc);
}

#[test]
fn test_with_capacity_and_hasher_in_mid_capacity() {
    let s = DefaultHashBuilder::default();
    let alloc = Global;
    let map = HashMap::with_capacity_and_hasher_in(5, s, alloc);
}

#[test]
fn test_with_capacity_and_hasher_in_max_capacity() {
    let s = DefaultHashBuilder::default();
    let alloc = Global;
    let map = HashMap::with_capacity_and_hasher_in(10, s, alloc);
}

#[test]
fn test_with_capacity_and_hasher_in_large_capacity() {
    let s = DefaultHashBuilder::default();
    let alloc = Global;
    let map = HashMap::with_capacity_and_hasher_in(10, s, alloc);
}

