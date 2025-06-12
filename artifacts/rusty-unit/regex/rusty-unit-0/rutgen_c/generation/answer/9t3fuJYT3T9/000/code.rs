// Answer 0

#[test]
fn test_resize_capacity_not_changed() {
    let mut threads = Threads::new();
    threads.resize(0, 1);
    assert_eq!(threads.slots_per_thread, 0);
    assert_eq!(threads.caps.len(), 0);
}

#[test]
fn test_resize_capacity_changed() {
    let mut threads = Threads::new();
    threads.resize(2, 1);
    assert_eq!(threads.slots_per_thread, 2);
    assert_eq!(threads.caps.len(), 4); // 2 * 2 (ncaps * 2)
    assert_eq!(threads.set.capacity(), 2);
}

#[test]
fn test_resize_multiple_slots() {
    let mut threads = Threads::new();
    threads.resize(3, 2);
    assert_eq!(threads.slots_per_thread, 4); // 2 * 2 (ncaps * 2)
    assert_eq!(threads.caps.len(), 12); // 3 * 4 (num_insts * slots_per_thread)
    assert_eq!(threads.set.capacity(), 3);
}

#[test]
fn test_resize_no_insts() {
    let mut threads = Threads::new();
    threads.resize(0, 3);
    assert_eq!(threads.slots_per_thread, 0);
    assert_eq!(threads.caps.len(), 0);
}

