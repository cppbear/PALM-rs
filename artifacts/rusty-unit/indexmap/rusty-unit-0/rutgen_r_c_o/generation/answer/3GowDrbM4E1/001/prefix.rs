// Answer 0

#[test]
fn test_is_subset_equal_length_non_empty() {
    let mut set1 = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    let mut set2 = IndexSet::with_capacity_and_hasher(5, RandomState::new());

    set1.insert(1);
    set1.insert(2);
    set2.insert(1);
    set2.insert(2);

    set1.is_subset(&set2);
}

#[test]
fn test_is_subset_equal_length_empty() {
    let set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    let set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());

    set1.is_subset(&set2);
}

#[test]
fn test_is_subset_length_zero_with_non_empty() {
    let set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    let mut set2 = IndexSet::with_capacity_and_hasher(5, RandomState::new());

    set2.insert(1);
    set2.insert(2);

    set1.is_subset(&set2);
}

#[test]
fn test_is_subset_with_identical_sets() {
    let mut set1 = IndexSet::with_capacity_and_hasher(3, RandomState::new());
    let mut set2 = IndexSet::with_capacity_and_hasher(3, RandomState::new());

    set1.insert(3);
    set1.insert(2);
    set1.insert(1);
    set2.insert(1);
    set2.insert(2);
    set2.insert(3);

    set1.is_subset(&set2);
}

#[test]
fn test_is_subset_large_equal_length() {
    let mut set1 = IndexSet::with_capacity_and_hasher(1000, RandomState::new());
    let mut set2 = IndexSet::with_capacity_and_hasher(1000, RandomState::new());

    for i in 0..1000 {
        set1.insert(i);
        set2.insert(i);
    }

    set1.is_subset(&set2);
}

#[test]
fn test_is_subset_with_empty_subsets() {
    let mut set1 = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    let mut set2 = IndexSet::with_capacity_and_hasher(5, RandomState::new());

    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    
    set2.insert(1);
    set2.insert(2);
    set2.insert(3);
    set2.insert(4);
    set2.insert(5);

    set1.is_subset(&set2);
}

