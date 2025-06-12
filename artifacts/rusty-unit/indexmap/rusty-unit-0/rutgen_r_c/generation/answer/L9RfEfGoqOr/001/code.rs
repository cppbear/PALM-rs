// Answer 0

#[test]
fn test_union_new_with_empty_sets() {
    use std::collections::hash_map::RandomState;

    let set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    let set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    
    let union = Union::new(&set1, &set2);
    assert!(union.iter.len() == 0);
}

#[test]
fn test_union_new_with_non_empty_set1() {
    use std::collections::hash_map::RandomState;

    let mut set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(2, RandomState::new());
    set1.insert(1);
    set1.insert(2);

    let set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());

    let union = Union::new(&set1, &set2);
    assert!(union.iter.len() == 2);
}

#[test]
fn test_union_new_with_non_empty_set2() {
    use std::collections::hash_map::RandomState;

    let set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());

    let mut set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(2, RandomState::new());
    set2.insert(3);
    set2.insert(4);

    let union = Union::new(&set1, &set2);
    assert!(union.iter.len() == 2);
}

#[test]
fn test_union_new_with_common_elements() {
    use std::collections::hash_map::RandomState;

    let mut set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(2, RandomState::new());
    set1.insert(1);
    set1.insert(2);

    let mut set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(2, RandomState::new());
    set2.insert(2);
    set2.insert(3);

    let union = Union::new(&set1, &set2);
    assert!(union.iter.len() == 3);
}

#[test]
fn test_union_new_with_large_sets() {
    use std::collections::hash_map::RandomState;

    let mut set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(1000, RandomState::new());
    let mut set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(1000, RandomState::new());

    for i in 0..500 {
        set1.insert(i);
    }
    for i in 250..750 {
        set2.insert(i);
    }

    let union = Union::new(&set1, &set2);
    assert!(union.iter.len() == 1000);
}

