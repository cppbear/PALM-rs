// Answer 0

#[test]
fn test_index_set_with_capacity_zero() {
    let set: IndexSet<u32, RandomState> = IndexSet::with_capacity(0);
    assert_eq!(set.map.core.len(), 0);
}

#[test]
fn test_index_set_with_capacity_non_zero() {
    let set: IndexSet<u32, RandomState> = IndexSet::with_capacity(5);
    assert_eq!(set.map.core.len(), 0);
}

#[test]
fn test_index_set_with_capacity_exceeding_initial_elements() {
    let set: IndexSet<u32, RandomState> = IndexSet::with_capacity(10);
    assert_eq!(set.map.core.len(), 0);
}

