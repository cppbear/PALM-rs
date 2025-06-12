// Answer 0

#[test]
fn test_with_capacity_zero() {
    let index_set: IndexSet<()> = IndexSet::with_capacity(0);
    // Check that the set is initialized without allocation.
    assert_eq!(index_set.map.core.len(), 0);
}

#[test]
fn test_with_capacity_small() {
    let n = 5;
    let index_set: IndexSet<()> = IndexSet::with_capacity(n);
    // Check that the set has capacity for 5 elements.
    assert!(index_set.map.core.capacity() >= n);
}

#[test]
fn test_with_capacity_large() {
    let n = 1000;
    let index_set: IndexSet<()> = IndexSet::with_capacity(n);
    // Check that the set has capacity for 1000 elements.
    assert!(index_set.map.core.capacity() >= n);
}

#[test]
fn test_with_capacity_boundary() {
    let n = usize::MAX;
    let index_set: IndexSet<()> = IndexSet::with_capacity(n);
    // On some systems, alloc may not support usize::MAX, 
    // but we will check if the function returns an IndexSet regardless.
    let _ = index_set; // Just ensure it compiles, we may not be able to assert on capacity here
}

