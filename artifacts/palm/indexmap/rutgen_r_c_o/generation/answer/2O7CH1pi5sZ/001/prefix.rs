// Answer 0

#[test]
fn test_with_capacity_and_hasher_zero() {
    let hash_builder = RandomState::new();
    let set = IndexSet::with_capacity_and_hasher(0, hash_builder);
}

#[test]
fn test_with_capacity_and_hasher_one() {
    let hash_builder = RandomState::new();
    let set = IndexSet::with_capacity_and_hasher(1, hash_builder);
}

#[test]
fn test_with_capacity_and_hasher_ten() {
    let hash_builder = RandomState::new();
    let set = IndexSet::with_capacity_and_hasher(10, hash_builder);
}

#[test]
fn test_with_capacity_and_hasher_hundred() {
    let hash_builder = RandomState::new();
    let set = IndexSet::with_capacity_and_hasher(100, hash_builder);
}

#[test]
fn test_with_capacity_and_hasher_thousand() {
    let hash_builder = RandomState::new();
    let set = IndexSet::with_capacity_and_hasher(1000, hash_builder);
}

#[test]
fn test_with_capacity_and_hasher_max() {
    let hash_builder = RandomState::new();
    let set = IndexSet::with_capacity_and_hasher(usize::MAX, hash_builder);
}

