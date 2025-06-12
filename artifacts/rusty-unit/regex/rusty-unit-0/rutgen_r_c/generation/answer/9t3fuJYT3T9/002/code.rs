// Answer 0

#[test]
fn test_resize_increases_capacity() {
    let mut threads = Threads::new();
    threads.resize(5, 2);
    assert_eq!(threads.set.capacity(), 5);
    assert_eq!(threads.slots_per_thread, 4);
    assert_eq!(threads.caps.len(), 20);
}

#[test]
fn test_resize_when_insts_equal_capacity() {
    let mut threads = Threads::new();
    threads.resize(5, 2); // First resize to set up capacity
    threads.resize(5, 3); // This should not alter the existing capacity
    assert_eq!(threads.set.capacity(), 5);
    assert_eq!(threads.slots_per_thread, 4);
    assert_eq!(threads.caps.len(), 20);
}

#[test]
fn test_resize_multiple_times() {
    let mut threads = Threads::new();
    threads.resize(3, 1);
    assert_eq!(threads.set.capacity(), 3);
    assert_eq!(threads.slots_per_thread, 2);
    assert_eq!(threads.caps.len(), 6);
    
    threads.resize(6, 2);
    assert_eq!(threads.set.capacity(), 6);
    assert_eq!(threads.slots_per_thread, 4);
    assert_eq!(threads.caps.len(), 24);

    threads.resize(4, 3);
    assert_eq!(threads.set.capacity(), 4);
    assert_eq!(threads.slots_per_thread, 6);
    assert_eq!(threads.caps.len(), 24); // The slots should keep the maximum
}

#[test]
fn test_resize_with_zero_ncaps() {
    let mut threads = Threads::new();
    threads.resize(5, 0);
    assert_eq!(threads.set.capacity(), 5);
    assert_eq!(threads.slots_per_thread, 0);
    assert_eq!(threads.caps.len(), 0);
}

