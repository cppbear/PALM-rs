// Answer 0

#[test]
fn test_resize_no_op() {
    struct SparseSet {
        capacity: usize,
    }
    
    impl SparseSet {
        fn new(capacity: usize) -> Self {
            SparseSet { capacity }
        }

        fn capacity(&self) -> usize {
            self.capacity
        }
    }

    struct TestStruct {
        set: SparseSet,
        slots_per_thread: usize,
        caps: Vec<Option<usize>>,
    }

    impl TestStruct {
        fn new(num_insts: usize) -> Self {
            let capacity = num_insts;
            let set = SparseSet::new(capacity);
            let slots_per_thread = 0; // Placeholder, will initialize later.
            let caps = Vec::new();
            TestStruct { set, slots_per_thread, caps }
        }

        fn resize(&mut self, num_insts: usize, ncaps: usize) {
            if num_insts == self.set.capacity() {
                return;
            }
            self.slots_per_thread = ncaps * 2;
            self.set = SparseSet::new(num_insts);
            self.caps = vec![None; self.slots_per_thread * num_insts];
        }
    }

    let mut test_instance = TestStruct::new(5);
    test_instance.resize(5, 3);

    assert_eq!(test_instance.slots_per_thread, 0); // Ensure it remains unchanged
    assert_eq!(test_instance.caps.len(), 0); // Ensure caps remains empty
}

#[test]
fn test_resize_change_capacity() {
    struct SparseSet {
        capacity: usize,
    }
    
    impl SparseSet {
        fn new(capacity: usize) -> Self {
            SparseSet { capacity }
        }

        fn capacity(&self) -> usize {
            self.capacity
        }
    }

    struct TestStruct {
        set: SparseSet,
        slots_per_thread: usize,
        caps: Vec<Option<usize>>,
    }

    impl TestStruct {
        fn new(num_insts: usize) -> Self {
            let capacity = num_insts;
            let set = SparseSet::new(capacity);
            let slots_per_thread = 0; // Placeholder
            let caps = Vec::new();
            TestStruct { set, slots_per_thread, caps }
        }

        fn resize(&mut self, num_insts: usize, ncaps: usize) {
            if num_insts == self.set.capacity() {
                return;
            }
            self.slots_per_thread = ncaps * 2;
            self.set = SparseSet::new(num_insts);
            self.caps = vec![None; self.slots_per_thread * num_insts];
        }
    }

    let mut test_instance = TestStruct::new(5);
    test_instance.resize(3, 3);

    assert_eq!(test_instance.slots_per_thread, 6); // Should be updated to ncaps * 2
    assert_eq!(test_instance.caps.len(), 18); // Should hold 18 elements
}

