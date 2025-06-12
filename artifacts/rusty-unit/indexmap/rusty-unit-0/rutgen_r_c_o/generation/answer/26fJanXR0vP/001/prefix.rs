// Answer 0

#[test]
fn test_is_empty_with_no_elements() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    assert!(set.is_empty());
}

#[test]
fn test_is_empty_after_adding_elements() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    set.map.insert(1, ());
    set.map.insert(2, ());
    assert!(!set.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    set.map.insert(1, ());
    set.clear();
    assert!(set.is_empty());
}

#[test]
fn test_is_empty_on_initialized_set() {
    let set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    assert!(set.is_empty());
}

#[test]
fn test_is_empty_after_truncate() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    set.map.insert(1, ());
    set.truncate(0);
    assert!(set.is_empty());
}

#[test]
fn test_is_empty_with_large_capacity() {
    let set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(100, RandomState::new());
    assert!(set.is_empty());
}

#[test]
#[should_panic]
fn test_is_empty_on_dropped_set() {
    let set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    std::mem::drop(set);
    set.is_empty();
}

