// Answer 0

#[derive(Debug)]
struct SparseSet {
    size: usize,
}

impl SparseSet {
    fn new(size: usize) -> Self {
        SparseSet { size }
    }
}

struct Threads {
    set: SparseSet,
    caps: Vec<u8>,
    slots_per_thread: usize,
}

impl Threads {
    fn new() -> Self {
        Threads {
            set: SparseSet::new(0),
            caps: vec![],
            slots_per_thread: 0,
        }
    }
}

#[test]
fn test_threads_new() {
    let threads = Threads::new();
    assert_eq!(threads.set.size, 0);
    assert!(threads.caps.is_empty());
    assert_eq!(threads.slots_per_thread, 0);
}

