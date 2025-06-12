// Answer 0

#[test]
fn test_with_capacity_and_hasher_small() {
    let n = 1;
    let hash_builder = RandomState::new();
    let map = IndexMap::with_capacity_and_hasher(n, hash_builder);
}

#[test]
fn test_with_capacity_and_hasher_medium() {
    let n = 100;
    let hash_builder = RandomState::new();
    let map = IndexMap::with_capacity_and_hasher(n, hash_builder);
}

#[test]
fn test_with_capacity_and_hasher_large() {
    let n = 1_000_000;
    let hash_builder = RandomState::new();
    let map = IndexMap::with_capacity_and_hasher(n, hash_builder);
}

#[test]
fn test_with_capacity_and_hasher_boundary_max() {
    let n = (2usize.pow(30)) - 1;
    let hash_builder = RandomState::new();
    let map = IndexMap::with_capacity_and_hasher(n, hash_builder);
}

#[should_panic]
fn test_with_capacity_and_hasher_panic() {
    let n = 0; // n == 0 is false for this function
    let hash_builder = RandomState::new();
    let _map = IndexMap::with_capacity_and_hasher(n, hash_builder);
}

