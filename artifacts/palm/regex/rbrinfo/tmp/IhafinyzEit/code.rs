fn new() -> Self {
        Threads {
            set: SparseSet::new(0),
            caps: vec![],
            slots_per_thread: 0,
        }
    }