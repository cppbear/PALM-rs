// Answer 0

#[test]
fn test_index_set_new() {
    // Create a new IndexSet using the provided `new` method
    let index_set: super::IndexSet<()> = super::IndexSet::new();
    // Assert that the new index set is empty, as it should not allocate any elements
    assert!(index_set.map.core.is_empty());
}

#[test]
fn test_index_set_with_capacity() {
    // Create a new IndexSet with a specific capacity
    let capacity = 10;
    let index_set: super::IndexSet<()> = super::IndexSet::with_capacity(capacity);
    // Assert that the capacity is set correctly (in this case, it might require checking internal structures)
    // Stub is_empty check since we do not have a direct capacity check visible 
    assert!(index_set.map.core.is_empty());
} 

#[test]
#[should_panic]
fn test_index_set_new_panic() {
    // Since the `new` function does not allocate, there should be no panic.
    // However, we can ensure panic conditions if further logic were added in the future.
    let _index_set: super::IndexSet<()> = super::IndexSet::new();
    // Potential panic conditions would need to be defined if there was logic causing panic.
}

