// Answer 0

#[test]
fn test_iter_empty() {
    let index_set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    let iter = index_set.iter();
}

#[test]
fn test_iter_single_element() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(1, RandomState::new());
    index_set.insert(1);
    let iter = index_set.iter();
}

#[test]
fn test_iter_multiple_elements() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    let iter = index_set.iter();
}

#[test]
fn test_iter_large_set() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10000, RandomState::new());
    for i in 0..10000 {
        index_set.insert(i);
    }
    let iter = index_set.iter();
}

#[test]
fn test_iter_non_empty_after_clear() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    index_set.insert(1);
    index_set.clear();
    index_set.insert(2);
    let iter = index_set.iter();
}

