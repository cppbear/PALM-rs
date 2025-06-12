// Answer 0

#[test]
fn test_resize_no_change() {
    let mut threads = Threads::new();
    let initial_capacity = threads.set.capacity();
    
    // Test with num_insts equal to current capacity
    threads.resize(initial_capacity, 5);
    
    // Verify slots_per_thread remains unchanged
    assert_eq!(threads.slots_per_thread, 0);
    // Verify SparseSet remains unchanged
    assert_eq!(threads.set.len(), 0);
    assert_eq!(threads.set.capacity(), initial_capacity);
}

#[test]
fn test_resize_increases_capacity() {
    let mut threads = Threads::new();
    threads.set = SparseSet::new(10); // Initial capacity of 10
    let new_capacity = 20;
    
    // Resize with num_insts greater than current capacity
    threads.resize(new_capacity, 5);
    
    // Verify slots_per_thread is updated
    assert_eq!(threads.slots_per_thread, 10);
    // Verify SparseSet is updated and now has a length of 0
    assert_eq!(threads.set.len(), 0);
    assert_eq!(threads.set.capacity(), new_capacity);
    // Verify caps is initialized properly
    assert_eq!(threads.caps.len(), 50); // 10 * 5 * 2 = 50
}

#[test]
fn test_resize_no_change_with_non_zero_caps() {
    let mut threads = Threads::new();
    threads.set = SparseSet::new(5); // Initial capacity of 5
    let initial_capacity = threads.set.capacity();
    
    // Test with num_insts equal to current capacity
    threads.resize(initial_capacity, 2);
    
    // Verify slots_per_thread should remain unchanged
    assert_eq!(threads.slots_per_thread, 0);
    // Verify SparseSet remains unchanged
    assert_eq!(threads.set.len(), 0);
    assert_eq!(threads.set.capacity(), initial_capacity);
}

#[test]
fn test_resize_changes_caps() {
    let mut threads = Threads::new();
    threads.set = SparseSet::new(7); // Initial capacity of 7
    let new_capacity = 15;
    
    // Resize with num_insts greater than current capacity
    threads.resize(new_capacity, 4);
    
    // Verify slots_per_thread is updated
    assert_eq!(threads.slots_per_thread, 8);
    // Verify SparseSet is now initialized properly
    assert_eq!(threads.set.len(), 0);
    assert_eq!(threads.set.capacity(), new_capacity);
    // Verify caps is initialized properly
    assert_eq!(threads.caps.len(), 120); // 15 * 4 * 2 = 120
}

