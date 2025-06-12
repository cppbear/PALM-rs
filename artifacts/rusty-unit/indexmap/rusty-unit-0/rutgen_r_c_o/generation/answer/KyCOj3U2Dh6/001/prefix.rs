// Answer 0

#[test]
fn test_new_index_set() {
    let index_set = IndexSet::new();
}

#[test]
fn test_new_index_set_with_capacity_zero() {
    let index_set = IndexSet::with_capacity(0);
}

#[test]
fn test_new_index_set_with_capacity_one() {
    let index_set = IndexSet::with_capacity(1);
}

#[test]
fn test_new_index_set_with_capacity_large() {
    let index_set = IndexSet::with_capacity(usize::MAX);
}

