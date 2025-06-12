// Answer 0

#[test]
fn test_hasher_default() {
    let hasher = DefaultHashBuilder::default();
    let set: HashSet<i32> = HashSet::with_hasher_in(hasher, Global);
    let _ = set.hasher();
}

#[test]
fn test_hasher_with_capacity_min() {
    let hasher = DefaultHashBuilder::default();
    let set: HashSet<i32> = HashSet::with_capacity_and_hasher_in(1, hasher, Global);
    let _ = set.hasher();
}

#[test]
fn test_hasher_with_capacity_small() {
    let hasher = DefaultHashBuilder::default();
    let set: HashSet<i32> = HashSet::with_capacity_and_hasher_in(10, hasher, Global);
    let _ = set.hasher();
}

#[test]
fn test_hasher_with_capacity_large() {
    let hasher = DefaultHashBuilder::default();
    let set: HashSet<i32> = HashSet::with_capacity_and_hasher_in(1000, hasher, Global);
    let _ = set.hasher();
}

#[test]
fn test_hasher_with_capacity_max() {
    let hasher = DefaultHashBuilder::default();
    let set: HashSet<i32> = HashSet::with_capacity_and_hasher_in(1_000_000, hasher, Global);
    let _ = set.hasher();
}

