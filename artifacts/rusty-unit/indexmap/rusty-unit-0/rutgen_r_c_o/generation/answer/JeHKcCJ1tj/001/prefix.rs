// Answer 0

#[test]
fn test_is_disjoint_equal_length_empty_sets() {
    let set_a: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    let set_b: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    set_a.is_disjoint(&set_b);
}

#[test]
fn test_is_disjoint_equal_length_no_common_elements() {
    let mut set_a: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(100, RandomState::new());
    let mut set_b: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(100, RandomState::new());
    set_a.insert(1);
    set_a.insert(2);
    set_b.insert(3);
    set_b.insert(4);
    set_a.is_disjoint(&set_b);
}

#[test]
fn test_is_disjoint_equal_length_all_elements_common() {
    let mut set_a: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(100, RandomState::new());
    let mut set_b: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(100, RandomState::new());
    set_a.insert(1);
    set_a.insert(2);
    set_b.insert(1);
    set_b.insert(2);
    set_a.is_disjoint(&set_b);
}

#[test]
fn test_is_disjoint_equal_length_some_common_elements() {
    let mut set_a: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(100, RandomState::new());
    let mut set_b: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(100, RandomState::new());
    set_a.insert(1);
    set_a.insert(2);
    set_b.insert(2);
    set_b.insert(3);
    set_a.is_disjoint(&set_b);
}

#[test]
fn test_is_disjoint_equal_length_large_sets() {
    let mut set_a: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(100, RandomState::new());
    let mut set_b: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(100, RandomState::new());
    for i in 0..50 {
        set_a.insert(i);
    }
    for i in 50..100 {
        set_b.insert(i);
    }
    set_a.is_disjoint(&set_b);
}

