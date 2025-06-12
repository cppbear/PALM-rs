// Answer 0

#[test]
fn test_truncate_zero_length() {
    let mut index_set = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    index_set.truncate(0);
}

#[test]
fn test_truncate_to_current_length() {
    let mut index_set = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    index_set.reserve(5);
    index_set.truncate(5);
}

#[test]
fn test_truncate_greater_than_current_length() {
    let mut index_set = IndexSet::with_capacity_and_hasher(3, RandomState::new());
    index_set.reserve(2);
    index_set.truncate(5);
}

#[test]
fn test_truncate_empty_set() {
    let mut index_set = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    index_set.truncate(0);
}

#[test]
fn test_truncate_to_one() {
    let mut index_set = IndexSet::with_capacity_and_hasher(3, RandomState::new());
    index_set.reserve(3);
    index_set.truncate(1);
}

#[test]
fn test_truncate_large_value() {
    let mut index_set = IndexSet::with_capacity_and_hasher(100, RandomState::new());
    index_set.reserve(50);
    index_set.truncate(usize::MAX);
}

