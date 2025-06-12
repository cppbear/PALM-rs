// Answer 0

#[test]
fn test_capacity_non_empty() {
    let set = IndexSet::with_capacity_and_hasher(100, RandomState::new());
    set.map.insert("item", ());
    let _ = set.capacity();
}

#[test]
fn test_capacity_empty() {
    let set = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    let _ = set.capacity();
}

#[test]
fn test_capacity_high_capacity() {
    let set = IndexSet::with_capacity_and_hasher(1000000, RandomState::new());
    let _ = set.capacity();
}

#[test]
fn test_capacity_specific_capacity() {
    let set = IndexSet::with_capacity_and_hasher(512, RandomState::new());
    let _ = set.capacity();
}

#[test]
fn test_capacity_after_insertions() {
    let mut set = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    for i in 0..5 {
        set.map.insert(i, ());
    }
    let _ = set.capacity();
}

#[test]
fn test_capacity_after_reserve() {
    let mut set = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    set.try_reserve(10).unwrap();
    let _ = set.capacity();
}

#[test]
fn test_capacity_after_shrink() {
    let mut set = IndexSet::with_capacity_and_hasher(100, RandomState::new());
    for i in 0..50 {
        set.map.insert(i, ());
    }
    set.shrink_to(10);
    let _ = set.capacity();
}

