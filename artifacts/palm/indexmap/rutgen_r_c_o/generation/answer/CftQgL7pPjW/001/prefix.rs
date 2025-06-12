// Answer 0

#[test]
fn test_swap_indices_valid() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    index_set.swap_indices(0, 1);
}

#[test]
fn test_swap_indices_edge_case_first_last() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    index_set.swap_indices(0, 2);
}

#[should_panic]
fn test_swap_indices_out_of_bounds_a() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    index_set.swap_indices(3, 1);
}

#[should_panic]
fn test_swap_indices_out_of_bounds_b() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    index_set.swap_indices(1, 3);
}

#[should_panic]
fn test_swap_indices_both_out_of_bounds() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet::new();
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    index_set.swap_indices(3, 4);
}

