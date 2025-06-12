// Answer 0

#[test]
fn test_clear_empty_set() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    set.clear();
}

#[test]
fn test_clear_small_set() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    set.map.insert(1, ());
    set.map.insert(2, ());
    set.map.insert(3, ());
    set.clear();
}

#[test]
fn test_clear_large_set() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(1_000_000, RandomState::new());
    for i in 0..1_000_000 {
        set.map.insert(i, ());
    }
    set.clear();
}

#[test]
fn test_clear_after_reserve() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    set.reserve(100);
    set.map.insert(5, ());
    set.map.insert(10, ());
    set.clear();
}

#[test]
fn test_clear_set_with_capacity() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(100, RandomState::new());
    set.map.insert(50, ());
    set.map.insert(60, ());
    set.clear();
}

#[test]
fn test_clear_set_with_different_hash_builder() {
    let hasher = RandomState::new();
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(20, hasher);
    set.map.insert(1, ());
    set.map.insert(2, ());
    set.clear();
}

