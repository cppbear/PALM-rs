// Answer 0

#[test]
fn test_is_disjoint_self_longer() {
    let mut self_set = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    let mut other_set = IndexSet::with_capacity_and_hasher(5, RandomState::new());

    for i in 0..10 {
        self_set.insert(i);
    }

    for i in 0..5 {
        other_set.insert(i + 10);
    }

    self_set.is_disjoint(&other_set);
}

#[test]
fn test_is_disjoint_self_longer_non_overlapping() {
    let mut self_set = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    let mut other_set = IndexSet::with_capacity_and_hasher(5, RandomState::new());

    for i in 0..10 {
        self_set.insert(i);
    }

    for i in 10..15 {
        other_set.insert(i);
    }

    self_set.is_disjoint(&other_set);
}

#[test]
fn test_is_disjoint_self_longer_overlap_edge_case() {
    let mut self_set = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    let mut other_set = IndexSet::with_capacity_and_hasher(5, RandomState::new());

    for i in 0..10 {
        self_set.insert(i);
    }

    for i in 0..10 {
        other_set.insert(i);
    }

    self_set.is_disjoint(&other_set);
}

#[test]
fn test_is_disjoint_self_longer_empty_other() {
    let mut self_set = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    let mut other_set = IndexSet::with_capacity_and_hasher(0, RandomState::new());

    for i in 0..5 {
        self_set.insert(i);
    }

    self_set.is_disjoint(&other_set);
}

#[test]
fn test_is_disjoint_self_longer_full_capacity() {
    let mut self_set = IndexSet::with_capacity_and_hasher(1000, RandomState::new());
    let mut other_set = IndexSet::with_capacity_and_hasher(500, RandomState::new());

    for i in 0..1000 {
        self_set.insert(i);
    }

    for i in 1000..1500 {
        other_set.insert(i);
    }

    self_set.is_disjoint(&other_set);
}

