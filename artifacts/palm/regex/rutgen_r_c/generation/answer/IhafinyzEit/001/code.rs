// Answer 0

#[test]
fn test_threads_new() {
    let threads = Threads::new();
    assert_eq!(threads.set.len(), 0);
    assert!(threads.set.is_empty());
    assert_eq!(threads.set.capacity(), 0);
    assert_eq!(threads.caps.len(), 0);
    assert_eq!(threads.slots_per_thread, 0);
}

