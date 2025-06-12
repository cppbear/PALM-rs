// Answer 0

#[test]
fn test_is_subset_when_empty() {
    use std::collections::hash_map::RandomState;

    let empty_set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    let result = empty_set.is_subset(&empty_set);
    assert!(result);
}

#[test]
fn test_is_subset_when_non_empty_subset() {
    use std::collections::hash_map::RandomState;

    let mut set_a: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(2, RandomState::new());
    set_a.insert(1);
    set_a.insert(2);

    let mut set_b: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(4, RandomState::new());
    set_b.insert(1);
    set_b.insert(2);
    set_b.insert(3);
    set_b.insert(4);

    let result = set_a.is_subset(&set_b);
    assert!(result);
}

#[test]
fn test_is_subset_when_not_subset() {
    use std::collections::hash_map::RandomState;

    let mut set_a: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(2, RandomState::new());
    set_a.insert(1);
    set_a.insert(2);

    let mut set_b: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(2, RandomState::new());
    set_b.insert(3);
    set_b.insert(4);

    let result = set_a.is_subset(&set_b);
    assert!(!result);
}

#[test]
fn test_is_subset_when_identical() {
    use std::collections::hash_map::RandomState;

    let mut set_a: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(2, RandomState::new());
    set_a.insert(1);
    set_a.insert(2);

    let mut set_b: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(2, RandomState::new());
    set_b.insert(1);
    set_b.insert(2);

    let result = set_a.is_subset(&set_b);
    assert!(result);
}

#[test]
fn test_is_subset_when_a_is_empty() {
    use std::collections::hash_map::RandomState;

    let empty_set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    let mut set_b: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(2, RandomState::new());
    set_b.insert(1);
    set_b.insert(2);

    let result = empty_set.is_subset(&set_b);
    assert!(result);
}

